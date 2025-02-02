use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_310828396: FileFormat = FileFormat {
    id: 310_828_396,
    source_type: SourceType::Linguist,
    name: "Gemini",
    extensions: &["gmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
