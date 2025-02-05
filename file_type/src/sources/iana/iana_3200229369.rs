use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3200229369: FileFormat = FileFormat {
    id: 3_200_229_369,
    source_type: SourceType::Iana,
    name: "flexfec",
    extensions: &[],
    media_types: &["text/flexfec"],
    signatures: &[],
    related_formats: &[],
};
