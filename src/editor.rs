use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::InterpolatorParams;

#[derive(Lens)]
struct Data {
    params: Arc<InterpolatorParams>
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 265))
}

pub(crate) fn create(
    params: Arc<InterpolatorParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Interpolator")
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Amount").child_top(Pixels(10.0));

            HStack::new(cx, |cx| {
                ParamSlider::new(cx, Data::params, |params| &params.amount).width(Pixels(128.0));
                ParamButton::new(cx,  Data::params, |params| &params.smooth).width(Pixels(44.0));
            }).height(Pixels(40.0));

            Label::new(cx, "Linear / Cubic");

            ParamSlider::new(cx, Data::params, |params| &params.cubic_correction).width(Pixels(196.0))
            .bottom(Pixels(10.0));

            Label::new(cx, "Tolerance");

            ParamSlider::new(cx, Data::params, |params| &params.tolerance).width(Pixels(196.0));

            Label::new(cx, "Version: 0.1.0")
            .font_size(11.0)
            .child_top(Pixels(20.0));
        
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
