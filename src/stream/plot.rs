use plotly::{
    layout::{GridPattern, Layout, LayoutGrid},
    Plot, Scatter,
};

pub fn mono_waveform(title: &str, signal: impl AsRef<[f32]>) {
    let signal = signal.as_ref();
    let mut plot = Plot::new();
    plot.add_trace(Scatter::new((0..signal.len()).collect(), signal.to_vec()));
    plot.set_layout(Layout::new().title(title.into()));
    plot.show();
}

pub fn stereo_waveform(title: &str, left: impl AsRef<[f32]>, right: impl AsRef<[f32]>) {
    let (l, r) = (left.as_ref(), right.as_ref());
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new((0..l.len()).collect(), l.to_vec())
            .x_axis("x1")
            .y_axis("y1"),
    );
    plot.add_trace(
        Scatter::new((0..r.len()).collect(), r.to_vec())
            .x_axis("x2")
            .y_axis("y2"),
    );
    plot.set_layout(
        Layout::new().title(title.into()).grid(
            LayoutGrid::new()
                .rows(2)
                .columns(1)
                .pattern(GridPattern::Independent),
        ),
    );
    plot.show();
}
