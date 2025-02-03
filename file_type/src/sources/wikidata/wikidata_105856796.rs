use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856796: FileFormat = FileFormat {
    id: 105_856_796,
    source_type: SourceType::Wikidata,
    name: "Klik'n'Play game",
    extensions: &["gam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x41, 0x4D, 0x45, 0x26, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
