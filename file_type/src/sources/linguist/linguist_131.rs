use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_131: FileType = FileType {
    file_format: &FileFormat {
        id: 131,
        source_type: SourceType::Linguist,
        name: "Gnuplot",
        extensions: &["gnu", "gnuplot", "gp", "p", "plot", "plt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
