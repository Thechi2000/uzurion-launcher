use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ErrorEventPayload {
    pub name: String,
    pub description: String,
}

#[macro_export]
macro_rules! send_event {
    ($app: expr, $event: expr, $payload: expr) => {
        {
            let __event = $event;
            let __payload = $payload;

            ::log::debug!("Sending event {} with payload {:?}", __event, ::serde_json::to_string(&__payload));
            if let Err(e) = ::tauri::Manager::emit_all(&$app, __event, __payload){
                ::log::error!("Could not send {} event: {:?}", __event, e);
            }
        }
    };
}

#[macro_export]
macro_rules! send_error {
    ($app: expr, $name: expr, $description: expr) => {
        crate::send_event!{
            $app,
            crate::consts::events::ERROR,
            crate::event::ErrorEventPayload{
                name: $name.to_string(),
                description: $description.to_string(),
            }
        }
    };
}