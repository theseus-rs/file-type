use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87894240: FileFormat = FileFormat {
    id: 87_894_240,
    source_type: SourceType::Wikidata,
    name: "Avery Label Pro Document 3",
    extensions: &["lpd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
