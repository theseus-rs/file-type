use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_295: FileFormat = FileFormat {
    id: 295,
    source_type: SourceType::Linguist,
    name: "Prolog",
    extensions: &["pl", "plt", "pro", "prolog", "yap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
