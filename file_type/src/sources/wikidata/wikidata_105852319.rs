use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852319: FileFormat = FileFormat {
    id: 105_852_319,
    source_type: SourceType::Wikidata,
    name: "Return To The Roots save game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x54, 0x54, 0x52, 0x53, 0x41, 0x56, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
