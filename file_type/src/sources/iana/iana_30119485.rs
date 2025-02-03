use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_30119485: FileFormat = FileFormat {
    id: 30_119_485,
    source_type: SourceType::Iana,
    name: "vnd.tencent.tap",
    extensions: &[],
    media_types: &["image/vnd.tencent.tap"],
    internal_signatures: &[],
    related_formats: &[],
};
