use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1601835: FileFormat = FileFormat {
    id: 1_601_835,
    source_type: SourceType::Wikidata,
    name: "Standard Test Data Format",
    extensions: &["std", "stdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
