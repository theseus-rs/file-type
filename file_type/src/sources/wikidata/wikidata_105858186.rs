use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858186: FileFormat = FileFormat {
    id: 105_858_186,
    source_type: SourceType::Wikidata,
    name: "ICE ECC data (v2.x)",
    extensions: &["ecc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x45, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
