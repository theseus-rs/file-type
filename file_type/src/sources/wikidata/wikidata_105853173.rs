use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853173: FileFormat = FileFormat {
    id: 105_853_173,
    source_type: SourceType::Wikidata,
    name: "STarKos song",
    extensions: &["sks"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x4B, 0x31, 0x2E, 0x30, 0x53, 0x4F, 0x4E, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
