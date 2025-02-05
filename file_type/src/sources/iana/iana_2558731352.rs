use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2558731352: FileFormat = FileFormat {
    id: 2_558_731_352,
    source_type: SourceType::Iana,
    name: "mizar",
    extensions: &[],
    media_types: &["text/mizar"],
    signatures: &[],
    related_formats: &[],
};
