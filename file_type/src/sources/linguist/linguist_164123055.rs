use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_164123055: FileFormat = FileFormat {
    id: 164_123_055,
    source_type: SourceType::Linguist,
    name: "SmPL",
    extensions: &["cocci"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
