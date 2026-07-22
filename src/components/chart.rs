use iced::{Element, Length};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::app::Message;

/// CPU History Chart
pub struct CpuChart<'a> {
    pub history: &'a [f32],
}

impl<'a> Chart<Message> for CpuChart<'a> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let max_len = self.history.len();
        if max_len == 0 {
            return;
        }

        let mut chart = match builder
            .margin(8)
            .x_label_area_size(0)
            .y_label_area_size(32)
            .build_cartesian_2d(0i32..(max_len as i32), 0.0f32..100.0f32)
        {
            Ok(c) => c,
            Err(_) => return,
        };

        let _ = chart.configure_mesh()
            .disable_x_mesh()
            .y_labels(5)
            .y_label_style(TextStyle::from(("sans-serif", 10).into_font()).color(&RGBAColor(180, 200, 220, 1.0)))
            .y_label_formatter(&|v| format!("{:.0}%", v))
            .axis_style(RGBAColor(50, 70, 90, 1.0))
            .draw();

        // Area fill
        let _ = chart.draw_series(
            AreaSeries::new(
                self.history.iter().enumerate().map(|(i, &v)| (i as i32, v)),
                0.0,
                RGBAColor(2, 180, 210, 0.15),
            )
        );

        // Line on top
        let _ = chart.draw_series(
            LineSeries::new(
                self.history.iter().enumerate().map(|(i, &v)| (i as i32, v)),
                ShapeStyle {
                    color: RGBAColor(2, 180, 210, 1.0),
                    filled: false,
                    stroke_width: 2,
                },
            )
        );
    }
}

pub fn cpu_chart_widget<'a>(history: &'a [f32]) -> Element<'a, Message> {
    ChartWidget::new(CpuChart { history })
        .width(Length::Fill)
        .height(Length::Fixed(160.0))
        .into()
}

/// RAM History Chart
pub struct RamChart<'a> {
    pub history: &'a [f32],
}

impl<'a> Chart<Message> for RamChart<'a> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let max_len = self.history.len();
        if max_len == 0 {
            return;
        }

        let mut chart = match builder
            .margin(8)
            .x_label_area_size(0)
            .y_label_area_size(32)
            .build_cartesian_2d(0i32..(max_len as i32), 0.0f32..100.0f32)
        {
            Ok(c) => c,
            Err(_) => return,
        };

        let _ = chart.configure_mesh()
            .disable_x_mesh()
            .y_labels(5)
            .y_label_style(TextStyle::from(("sans-serif", 10).into_font()).color(&RGBAColor(180, 200, 220, 1.0)))
            .y_label_formatter(&|v| format!("{:.0}%", v))
            .axis_style(RGBAColor(50, 70, 90, 1.0))
            .draw();

        // Area fill - purple for RAM
        let _ = chart.draw_series(
            AreaSeries::new(
                self.history.iter().enumerate().map(|(i, &v)| (i as i32, v)),
                0.0,
                RGBAColor(130, 80, 220, 0.15),
            )
        );

        // Line on top
        let _ = chart.draw_series(
            LineSeries::new(
                self.history.iter().enumerate().map(|(i, &v)| (i as i32, v)),
                ShapeStyle {
                    color: RGBAColor(160, 100, 255, 1.0),
                    filled: false,
                    stroke_width: 2,
                },
            )
        );
    }
}

pub fn ram_chart_widget<'a>(history: &'a [f32]) -> Element<'a, Message> {
    ChartWidget::new(RamChart { history })
        .width(Length::Fill)
        .height(Length::Fixed(160.0))
        .into()
}
