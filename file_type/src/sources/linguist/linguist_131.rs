use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_131: FileFormat = FileFormat {
    id: 131,
    source_type: SourceType::Linguist,
    name: "Gnuplot",
    extensions: &["gnu", "gnuplot", "gp", "p", "plot", "plt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
