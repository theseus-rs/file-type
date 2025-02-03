use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_813068465: FileFormat = FileFormat {
    id: 813_068_465,
    source_type: SourceType::Linguist,
    name: "Noir",
    extensions: &["nr"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
