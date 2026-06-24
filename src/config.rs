// This file is copied and modified from https://github.com/pop-os/cosmic-comp/blob/aac1e19f08a016ade349569fbf8c0305761de20b/cosmic-comp-config/src/input.rs
// SPDX-License-Identifier: GPL-3.0-only

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

// Note: For the following values, None is used to represent the system default
// Configuration for input devices
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputConfig {
    pub state: DeviceState,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub acceleration: Option<AccelConfig>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub calibration: Option<[f32; 6]>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub click_method: Option<ClickMethod>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub disable_while_typing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub left_handed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub middle_button_emulation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rotation_angle: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scroll_config: Option<ScrollConfig>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tap_config: Option<TapConfig>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub map_to_output: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccelConfig {
    pub profile: Option<AccelProfile>,
    pub speed: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ScrollConfig {
    pub method: Option<ScrollMethod>,
    pub natural_scroll: Option<bool>,
    pub scroll_button: Option<u32>,
    pub scroll_factor: Option<f64>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum DeviceState {
    #[default]
    Enabled,
    Disabled,
    DisabledOnExternalMouse,
}

// #[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum TouchpadOverride {
//     #[default]
//     None,
//     ForceDisable,
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TapConfig {
    pub enabled: bool,
    pub button_map: Option<TapButtonMap>,
    pub drag: bool,
    pub drag_lock: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ClickMethod {
    ButtonAreas,
    Clickfinger,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AccelProfile {
    Flat,
    Adaptive,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ScrollMethod {
    NoScroll,
    TwoFinger,
    Edge,
    OnButtonDown,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TapButtonMap {
    LeftRightMiddle,
    LeftMiddleRight,
}
