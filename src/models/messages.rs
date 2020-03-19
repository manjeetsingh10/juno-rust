use serde_json::{Map, Value};

use std::collections::HashMap;

#[allow(dead_code)]
pub enum BaseMessage {
	RegisterModuleRequest {
		request_id: String,
		module_id: String,
		version: String,
		dependencies: HashMap<String, String>,
	},
	RegisterModuleResponse {
		request_id: String,
	},
	FunctionCallRequest {
		request_id: String,
		function: String,
		arguments: Map<String, Value>,
	},
	FunctionCallResponse {
		request_id: String,
		data: Value,
	},
	RegisterHookRequest {
		request_id: String,
		hook: String,
	},
	ListenHookResponse {
		request_id: String,
	},
	TriggerHookRequest {
		request_id: String,
		hook: String,
	},
	TriggerHookResponse {
		request_id: String,
	},
	DeclareFunctionRequest {
		request_id: String,
		function: String,
	},
	DeclareFunctionResponse {
		request_id: String,
		function: String,
	},
	Unknown {
		request_id: String,
	},
}

impl BaseMessage {
	pub fn get_type(&self) -> u64 {
		match &self {
			BaseMessage::Unknown { .. } => 0,
			BaseMessage::RegisterModuleRequest { .. } => 1,
			BaseMessage::RegisterModuleResponse { .. } => 2,
			BaseMessage::FunctionCallRequest { .. } => 3,
			BaseMessage::FunctionCallResponse { .. } => 4,
			BaseMessage::RegisterHookRequest { .. } => 5,
			BaseMessage::ListenHookResponse { .. } => 6,
			BaseMessage::TriggerHookRequest { .. } => 7,
			BaseMessage::TriggerHookResponse { .. } => 8,
			BaseMessage::DeclareFunctionRequest { .. } => 9,
			BaseMessage::DeclareFunctionResponse { .. } => 10,
		}
	}

	pub fn get_request_id(&self) -> &String {
		match &self {
			BaseMessage::Unknown { request_id } => request_id,
			BaseMessage::RegisterModuleRequest { request_id, .. } => request_id,
			BaseMessage::RegisterModuleResponse { request_id, .. } => request_id,
			BaseMessage::FunctionCallRequest { request_id, .. } => request_id,
			BaseMessage::FunctionCallResponse { request_id, .. } => request_id,
			BaseMessage::RegisterHookRequest { request_id, .. } => request_id,
			BaseMessage::ListenHookResponse { request_id, .. } => request_id,
			BaseMessage::TriggerHookRequest { request_id, .. } => request_id,
			BaseMessage::TriggerHookResponse { request_id, .. } => request_id,
			BaseMessage::DeclareFunctionRequest { request_id, .. } => request_id,
			BaseMessage::DeclareFunctionResponse { request_id, .. } => request_id,
		}
	}
}
