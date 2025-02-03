use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3225988671: FileFormat = FileFormat {
    id: 3_225_988_671,
    source_type: SourceType::Iana,
    name: "3gpp2",
    extensions: &[],
    media_types: &["video/3gpp2"],
    internal_signatures: &[],
    related_formats: &[],
};
