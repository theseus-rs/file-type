use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2022828087: FileFormat = FileFormat {
    id: 2_022_828_087,
    source_type: SourceType::Iana,
    name: "vnd.framemaker",
    extensions: &[],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
