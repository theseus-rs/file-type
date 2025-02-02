use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853074: FileFormat = FileFormat {
    id: 105_853_074,
    source_type: SourceType::Wikidata,
    name: "Unreal Engine 4 save game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x56, 0x41, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
