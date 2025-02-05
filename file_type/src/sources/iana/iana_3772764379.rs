use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3772764379: FileFormat = FileFormat {
    id: 3_772_764_379,
    source_type: SourceType::Iana,
    name: "vnd.dolby.heaac.1",
    extensions: &[],
    media_types: &["audio/vnd.dolby.heaac.1"],
    signatures: &[],
    related_formats: &[],
};
