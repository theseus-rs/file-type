use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_291: FileFormat = FileFormat {
    id: 291,
    source_type: SourceType::Linguist,
    name: "PostScript",
    extensions: &["eps", "epsi", "pfa", "ps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
