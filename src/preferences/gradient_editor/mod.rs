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
        obj.imp().new_stop_button.connect_clicked(clone!(@weak obj => move |_| {
            obj.append_stop();
        }));
        obj.imp().stops_notebook.connect_page_added(move |nb,_child,_num| {
            for n in 0..nb.n_pages() {
                nb.nth_page(Some(n))
                    .unwrap()
                    .downcast::<StopEditor>()
                    .unwrap()
                    .set_button_visible(nb.n_pages() > 2);
            }
        });
        obj.imp().stops_notebook.connect_page_removed(move |nb,_child,_num| {
            for n in 0..nb.n_pages() {
                nb.nth_page(Some(n))
                    .unwrap()
                    .downcast::<StopEditor>()
                    .unwrap()
                    .set_button_visible(nb.n_pages() > 2);
            }
        });
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
        let mut stops: Vec<Stop> = vec![];
        for n in 0..self.imp().stops_notebook.n_pages() {
            stops.push(
                self.imp()
                    .stops_notebook
                    .nth_page(Some(n))
                    .unwrap()
                    .downcast::<StopEditor>()
                    .unwrap()
                    .values(),
            );
        }
        stops.sort_by(|a, b| a.partial_cmp(b).unwrap());
        stops
    }

    pub fn set_stops(&self, stops: &[Stop]) {
        let mut stops: Vec<&Stop> = stops.iter().collect();
        stops.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for s in &stops {
            let stop_editor = StopEditor::new_with_stop(s);
            stop_editor.imp().label.imp().button.set_visible(stops.len() > 2);
            self.imp()
                .stops_notebook
                .append_page(&stop_editor, Some(&stop_editor.imp().label));
            let nb = self.imp().stops_notebook.clone();
            stop_editor.imp().label.imp().button.connect_clicked(
                clone!(@weak stop_editor => move |_| {
                    nb.remove_page(nb.page_num(&stop_editor));
                }),
            );
        }
    }

    pub fn append_stop(&self) -> StopEditor {
        let stop_editor = StopEditor::new();
        self.imp()
            .stops_notebook
            .append_page(&stop_editor, Some(&stop_editor.imp().label));
        let nb = self.imp().stops_notebook.clone();
        stop_editor.imp().label.imp().button.connect_clicked(
            clone!(@weak stop_editor => move |_| {
                nb.remove_page(nb.page_num(&stop_editor));
            }),
        );
        stop_editor
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
