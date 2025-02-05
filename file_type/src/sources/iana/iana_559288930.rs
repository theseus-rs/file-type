use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_559288930: FileFormat = FileFormat {
    id: 559_288_930,
    source_type: SourceType::Iana,
    name: "vnd.ms-powerpoint.addin.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-powerpoint.addin.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};
