use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3917093790: FileFormat = FileFormat {
    id: 3_917_093_790,
    source_type: SourceType::Iana,
    name: "vnd.rosette.annotated-data-model",
    extensions: &[],
    media_types: &["model/vnd.rosette.annotated-data-model"],
    signatures: &[],
    related_formats: &[],
};
