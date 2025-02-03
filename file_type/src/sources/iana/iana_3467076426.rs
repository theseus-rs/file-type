use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3467076426: FileFormat = FileFormat {
    id: 3_467_076_426,
    source_type: SourceType::Iana,
    name: "parityfec",
    extensions: &[],
    media_types: &["application/parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
