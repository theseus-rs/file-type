use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856046: FileFormat = FileFormat {
    id: 105_856_046,
    source_type: SourceType::Wikidata,
    name: "Adventure Game Toolkit strings temporary data",
    extensions: &["d$$"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x3E, 0x3E, 0x20, 0x54, 0x65, 0x6D, 0x70, 0x6F, 0x72, 0x61, 0x72, 0x79,
                    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x6F, 0x66,
                    0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x73, 0x20, 0x66, 0x6F, 0x72, 0x20,
                    0x41, 0x47, 0x54, 0x20, 0x3C, 0x3C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
