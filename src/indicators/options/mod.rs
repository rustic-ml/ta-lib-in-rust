//! # Options Market Indicators
//!
//! This module provides indicators specialized for options markets.
//!
//! ## Available Indicator Groups
//!
//! - [`implied_volatility`](implied_volatility/index.html): Indicators based on implied volatility analysis
//! - [`greeks`](greeks/index.html): Indicators and calculations for option Greeks

pub mod greeks;
pub mod implied_volatility;

// Re-export common types and functions for convenient access
pub use greeks::GreeksCalculator;
pub use implied_volatility::IVSurface;
