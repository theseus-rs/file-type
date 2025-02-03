use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3932422767: FileFormat = FileFormat {
    id: 3_932_422_767,
    source_type: SourceType::Iana,
    name: "x-mixed-replace",
    extensions: &[],
    media_types: &["multipart/x-mixed-replace"],
    internal_signatures: &[],
    related_formats: &[],
};
