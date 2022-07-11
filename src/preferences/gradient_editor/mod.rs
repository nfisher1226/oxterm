mod imp;
mod stop_editor;

use {
    crate::{config::Stop, Values},
    gtk::{
        glib::{self, GString, clone, Object},
        prelude::*,
        subclass::prelude::*,
    },
    rgba_simple::PrimaryColor,
    stop_editor::StopEditor,
};

glib::wrapper! {
    pub struct GradientEditor(ObjectSubclass<imp::GradientEditor>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for GradientEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientEditor {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self = Object::new(&[]).expect("Cannot create gradient editor");
        let imp = obj.imp();
        let stop = obj.append_stop();
        stop.set_values(&Stop::new(PrimaryColor::Black.into(), Stop::MIN_POSITION));
        imp.stop_selector.set_active_id(Some(stop.widget_name().as_str()));
        let stop = obj.append_stop();
        stop.set_values(&Stop::new(PrimaryColor::White.into(), Stop::MAX_POSITION));
        imp.gradient_kind.connect_changed(clone!(@strong obj as s => move |gk| {
            let stack = &s.imp().position_type_stack;
            match gk.active_id().unwrap_or(GString::from("")).as_str() {
                "linear" => stack.set_visible_child_name("end_position"),
                "elliptical" | "radial" => {
                    stack.set_visible_child_name("start_position");
                    s.imp().direction_stack.set_visible_child_name("edge_page");
                },
                _ => {},
            }
        }));
        imp.direction_type.connect_changed(clone!(@strong obj as s => move |dt| {
            let stack = &s.imp().direction_stack;
            match dt.active_id().unwrap_or(GString::from("")).as_str() {
                "angle" => stack.set_visible_child_name("angle_page"),
                "edge" => stack.set_visible_child_name("edge_page"),
                _ => {},
            }
        }));
        imp.stop_selector.connect_changed(clone!(@strong obj as s => move |sel| {
            if let Some(name) = sel.active_id() {
                if let Some(stop) = s.imp().stops.borrow().get(name.as_str()) {
                    s.imp().stops_stack.set_visible_child(stop);
                }
            }
        }));
        imp.num_stops.connect_value_changed(clone!(@strong obj as s => move |ns| {
            let old = { s.imp().stops.borrow().values().len() };
            let new = ns.value();
            if new > old as f64 {
                let _stop = s.append_stop();
            } else if new < old as f64 {
                if let Some(name) = s.imp().stop_selector.active_id() {
                    s.remove_stop(name.as_str());
                }
            }
        }));
        obj
    }

    pub fn append_stop(&self) -> StopEditor {
        let stop_editor = StopEditor::new();
        let name = stop_editor.widget_name().to_string();
        let _page = self.imp().stops_stack.add_child(&stop_editor);
        self.imp()
            .stops
            .borrow_mut()
            .insert(stop_editor.widget_name().to_string(), stop_editor.clone());
        self.imp().stop_selector.append(Some(&name), &name);
        stop_editor
    }

    pub fn remove_stop(&self, id: &str) {
        let imp = self.imp();
        let stop = { imp.stops.borrow_mut().remove(id) };
        if let Some(stop) = stop {
            imp.stops_stack.remove(&stop);
            imp.stop_selector.remove_all();
            for name in imp.stops.borrow().keys() {
                let _stop = imp.stop_selector.append(Some(name), name);
            }
        }
    }
}
