use leptos::*;

use crate::components::context::LeafletMapContext;
use crate::components::context::TileLayerWmsContext;
use crate::MapEvents;
use leaflet::{Map, TileLayerWms as LeafletTileLayerWms, TileLayerWmsOptions};

#[component(transparent)]
pub fn TileLayerWms(
    #[prop(into)] url: String,
    options: TileLayerWmsOptions,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    let wms_context = TileLayerWmsContext::new();
    provide_context(wms_context);

    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            let map_layer = leaflet::TileLayerWms::new_options(&url, &options);
            map_layer.add_to(&map);
            wms_context.set_wms(&map_layer);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
    children.map_or(view! { <>""</> }, |c| view! { <>{ c() }</>})
}

#[component(transparent)]
pub fn TileLayerWmsEvents<F>(map_events: F) -> impl IntoView
where
    F: Fn(&Map, &LeafletTileLayerWms) -> MapEvents + 'static,
{
    let map_ctx =
        use_context::<crate::LeafletMapContext>().expect("LeafletMapContext not available.");
    let wms_ctx =
        use_context::<crate::TileLayerWmsContext>().expect("TileLayerWmsContext not available.");

    create_effect(move |_| {
        let map = map_ctx.map();
        let wms = wms_ctx.wms();
        if let (Some(m), Some(w)) = (map, wms) {
            let events = map_events(&m, &w);
            events.setup(&m);
        }
    });
}
