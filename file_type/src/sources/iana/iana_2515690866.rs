use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2515690866: FileFormat = FileFormat {
    id: 2_515_690_866,
    source_type: SourceType::Iana,
    name: "vnd.olpc-sugar",
    extensions: &[],
    media_types: &["application/vnd.olpc-sugar"],
    internal_signatures: &[],
    related_formats: &[],
};
