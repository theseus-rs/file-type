use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851719: FileFormat = FileFormat {
    id: 105_851_719,
    source_type: SourceType::Wikidata,
    name: "StarCraft 2 Localization Header",
    extensions: &["s2qh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x22, 0x6D, 0x5F, 0x68, 0x61, 0x6E, 0x64, 0x6C, 0x65, 0x22, 0x3A, 0x7B,
                    0x22, 0x6D, 0x5F, 0x69, 0x64, 0x22, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
