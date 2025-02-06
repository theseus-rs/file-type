use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_952972794: FileFormat = FileFormat {
    id: 952_972_794,
    source_type: SourceType::Linguist,
    name: "ZAP",
    extensions: &["xzap", "zap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
