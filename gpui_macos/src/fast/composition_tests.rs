// Modified by Pioneer for the macOS single-layer scene fast path.
use super::single_layer_scene;
use gpui::{
    AppContext as _, Bounds, ComposedSceneLayer, CompositionSurfaceId, ContentMask, Context,
    DevicePixels, IntoElement, ParentElement as _, PlatformHeadlessRenderer as _, Quad, Render,
    ScaledPixels, Scene, Styled as _, TestAppContext, Window, div, point, px, size,
};

use crate::metal_renderer::MetalHeadlessRenderer;

#[gpui::test]
fn single_layer_rounded_scene_matches_balanced_replay(cx: &mut TestAppContext) {
    struct View;
    impl Render for View {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size(px(64.)).bg(gpui::white()).child(
                div()
                    .size(px(64.))
                    .rounded(px(24.))
                    .overflow_hidden()
                    .child(div().size(px(64.)).bg(gpui::black())),
            )
        }
    }

    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(|_| View))
            .unwrap()
    });
    let mut renderer = MetalHeadlessRenderer::new();
    window
        .update(cx, |_, window, cx| {
            window.draw(cx).clear(cx);
            let scene = window.painted_scene();
            assert!(
                !scene.layers.frames.is_empty(),
                "rounded clips were compiled"
            );
            let layer = ComposedSceneLayer {
                surface: CompositionSurfaceId::default(),
                ranges: vec![0..scene.len()],
            };
            let layers = [layer];
            let borrowed = single_layer_scene(scene, &layers).unwrap();
            assert!(std::ptr::eq(borrowed, scene));

            // The existing layer_scene path replays the original operations,
            // including rounded clips, and finishes the new surface scene.
            let mut replayed = Scene::default();
            replayed
                .replay_balanced(layers[0].ranges[0].clone(), scene)
                .unwrap();
            replayed.finish();

            let side = (64. * window.scale_factor()) as i32;
            let extent = size(DevicePixels(side), DevicePixels(side));
            let expected = renderer.render_scene_to_image(&replayed, extent).unwrap();
            let actual = renderer.render_scene_to_image(borrowed, extent).unwrap();
            assert_eq!(actual.as_raw(), expected.as_raw());
            assert_eq!(actual.get_pixel(1, 1).0, [255, 255, 255, 255]);
            assert_eq!(
                actual.get_pixel(side as u32 / 2, side as u32 / 2).0,
                [0, 0, 0, 255]
            );
        })
        .unwrap();
}

#[test]
fn partial_or_split_layers_require_replay_even_when_primitives_match() {
    let mut scene = Scene::default();
    let bounds = Bounds::new(
        point(ScaledPixels(0.), ScaledPixels(0.)),
        size(ScaledPixels(64.), ScaledPixels(64.)),
    );
    scene.push_layer(bounds);
    scene.insert_primitive(Quad {
        bounds,
        content_mask: ContentMask { bounds },
        ..Default::default()
    });
    scene.pop_layer();
    scene.finish();
    let len = scene.len();
    let layer = |ranges| ComposedSceneLayer {
        surface: CompositionSurfaceId::default(),
        ranges,
    };
    // The inner range has the same sole primitive, but must restore its layer.
    for layers in [
        vec![layer(vec![1..len - 1])],
        vec![layer(vec![0..len - 1])],
        vec![layer(vec![1..len])],
        vec![layer(vec![0..1, 1..len])],
        vec![layer(vec![0..len]), layer(vec![])],
        vec![layer(vec![])],
        vec![layer(vec![0..len + 1])],
        vec![],
    ] {
        assert!(single_layer_scene(&scene, &layers).is_none(), "{layers:?}");
    }

    let mut empty = Scene::default();
    empty.finish();
    assert!(single_layer_scene(&empty, &[layer(vec![])]).is_none());
    assert!(std::ptr::eq(
        single_layer_scene(&empty, &[layer(vec![0..0])]).unwrap(),
        &empty,
    ));
}
