use crate::engine::services::input_service::InputService;

/// Contains all services usable by the engine and provides easy access to said services
///
/// Technically anti-pattern due to hiding dependencies as an entity could for example make use a
/// render_service and the inpu_service without directly stating so.
pub struct ServiceLocator {
    input_service: InputService,
}
