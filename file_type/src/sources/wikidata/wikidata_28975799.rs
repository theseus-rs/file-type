use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975799: FileFormat = FileFormat {
    id: 28_975_799,
    source_type: SourceType::Wikidata,
    name: "FACT",
    extensions: &["fact"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
