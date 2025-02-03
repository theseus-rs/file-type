use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861981: FileFormat = FileFormat {
    id: 105_861_981,
    source_type: SourceType::Wikidata,
    name: "Trilo Tracker Macro",
    extensions: &["ma"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
