use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_219983: FileFormat = FileFormat {
    id: 219_983,
    source_type: SourceType::Wikidata,
    name: "Zoo",
    extensions: &["zoo"],
    media_types: &["application/x-zoo"],
    signatures: &[],
    related_formats: &[],
};
