mod imp;
mod stop_editor;

use {
    crate::{
        config::{
            Direction, Gradient, GradientKind, HorizontalPlacement, Placement, Stop,
            VerticalPlacement,
        },
        Values,
    },
    gtk::{
        glib::{self, clone, GString, Object},
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

#[allow(clippy::cast_precision_loss)]
impl GradientEditor {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self = Object::new(&[]).expect("Cannot create gradient editor");
        let imp = obj.imp();
        let stop = obj.append_stop();
        stop.set_values(&Stop::new(PrimaryColor::Black.into(), Stop::MIN_POSITION));
        imp.stop_selector
            .set_active_id(Some(stop.widget_name().as_str()));
        let stop = obj.append_stop();
        stop.set_values(&Stop::new(PrimaryColor::White.into(), Stop::MAX_POSITION));
        imp.num_stops
            .connect_value_changed(clone!(@strong obj as editor => move |ns| {
                let old = { editor.imp().stops.borrow().values().len() };
                let new = ns.value();
                if new > old as f64 {
                    let _stop = editor.append_stop();
                } else if new < old as f64 {
                    if let Some(name) = editor.imp().stop_selector.active_id() {
                        editor.remove_stop(name.as_str());
                    }
                }
            }));
        obj
    }

    fn horizontal_placement(&self) -> Option<HorizontalPlacement> {
        if let Some(s) = self.imp().horizontal_position.active_id() {
            if let Ok(placement) = s.parse() {
                return Some(placement);
            }
        }
        None
    }

    fn set_horizontal_placement(&self, placement: &HorizontalPlacement) {
        self.imp()
            .horizontal_position
            .set_active_id(Some(&placement.to_string()));
    }

    fn vertical_placement(&self) -> Option<VerticalPlacement> {
        if let Some(s) = self.imp().vertical_position.active_id() {
            if let Ok(placement) = s.parse() {
                return Some(placement);
            }
        }
        None
    }

    fn set_vertical_placement(&self, placement: &VerticalPlacement) {
        self.imp()
            .vertical_position
            .set_active_id(Some(&placement.to_string()));
    }

    pub fn placement(&self) -> Placement {
        Placement {
            vertical: self.vertical_placement().unwrap_or_default(),
            horizontal: self.horizontal_placement().unwrap_or_default(),
        }
    }

    pub fn set_placement(&self, placement: &Placement) {
        self.set_horizontal_placement(&placement.horizontal);
        self.set_vertical_placement(&placement.vertical);
    }

    pub fn degrees(&self) -> f64 {
        self.imp().degrees.value()
    }

    pub fn set_degrees(&self, degrees: f64) {
        self.imp().degrees.set_value(degrees);
    }

    pub fn direction(&self) -> Direction {
        match self
            .imp()
            .direction_type
            .active_id()
            .unwrap_or_else(|| GString::from(""))
            .as_str()
        {
            "angle" => Direction::Angle(self.degrees()),
            "edge" => Direction::Edge(self.placement()),
            _ => Direction::default(),
        }
    }

    pub fn set_direction(&self, direction: &Direction) {
        match direction {
            Direction::Angle(degrees) => {
                self.set_degrees(*degrees);
                self.imp().direction_type.set_active_id(Some("angle"));
                self.imp().direction_stack.set_visible_child_name("angle");
            }
            Direction::Edge(edge) => {
                self.set_placement(edge);
                self.imp().direction_type.set_active_id(Some("edge"));
                self.imp().direction_stack.set_visible_child_name("edge");
            }
        }
    }

    pub fn stops(&self) -> Vec<Stop> {
        let mut stops = self
            .imp()
            .stops
            .borrow()
            .values()
            .map(Values::values)
            .collect::<Vec<Stop>>();
        stops.sort_by(|a, b| a.partial_cmp(b).unwrap());
        stops
    }

    pub fn set_stops(&self, stops: &[Stop]) {
        {
            let mut s = self.imp().stops.borrow_mut();
            for stop in s.values() {
                self.imp().stops_stack.remove(stop);
            }
            s.drain();
        }
        self.imp().stop_selector.remove_all();
        self.imp().num_stops.set_value(stops.len() as f64);
        for s in stops {
            let stop_editor = StopEditor::new_with_stop(s);
            let name = stop_editor.widget_name().to_string();
            let _page = self.imp().stops_stack.add_named(&stop_editor, Some(&name));
            self.imp().stops_stack.set_visible_child_name(&name);
            self.imp().stop_selector.append(Some(&name), &name);
            self.imp().stop_selector.set_active_id(Some(&name));
            self.imp().stops.borrow_mut().insert(name, stop_editor);
        }
    }

    pub fn append_stop(&self) -> StopEditor {
        let stop_editor = StopEditor::new();
        let name = stop_editor.widget_name().to_string();
        let _page = self.imp().stops_stack.add_named(&stop_editor, Some(&name));
        self.imp().stop_selector.append(Some(&name), &name);
        self.imp()
            .stops
            .borrow_mut()
            .insert(name, stop_editor.clone());
        stop_editor
    }

    pub fn remove_stop(&self, id: &str) {
        let imp = self.imp();
        let stop = { imp.stops.borrow_mut().remove(id) };
        if let Some(stop) = stop {
            imp.stops_stack.remove(&stop);
            imp.stop_selector.remove_all();
            for name in imp.stops.borrow().keys() {
                imp.stop_selector.append(Some(name), name);
            }
        }
    }
}

impl Values<Gradient> for GradientEditor {
    fn values(&self) -> Gradient {
        match self
            .imp()
            .gradient_kind
            .active_id()
            .unwrap_or_else(|| GString::from(""))
            .as_str()
        {
            "linear" => Gradient {
                kind: GradientKind::Linear(self.direction()),
                stops: self.stops(),
            },
            "radial" => Gradient {
                kind: GradientKind::Radial(self.placement()),
                stops: self.stops(),
            },
            "elliptical" => Gradient {
                kind: GradientKind::Elliptical(self.placement()),
                stops: self.stops(),
            },
            _ => Gradient {
                kind: GradientKind::default(),
                stops: self.stops(),
            },
        }
    }

    fn set_values(&self, gradient: &Gradient) {
        match &gradient.kind {
            GradientKind::Linear(direction) => {
                self.imp().gradient_kind.set_active_id(Some("linear"));
                self.set_direction(direction);
            }
            GradientKind::Radial(placement) => {
                self.imp().gradient_kind.set_active_id(Some("radial"));
                self.set_placement(placement);
            }
            GradientKind::Elliptical(placement) => {
                self.imp().gradient_kind.set_active_id(Some("elliptical"));
                self.set_placement(placement);
            }
        }
        self.set_stops(&gradient.stops);
    }
}
