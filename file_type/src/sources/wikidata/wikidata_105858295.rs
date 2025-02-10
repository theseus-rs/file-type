use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858295: FileFormat = FileFormat {
    id: 105_858_295,
    source_type: SourceType::Wikidata,
    name: "Eureka/Mercury Configuration",
    extensions: &["cfg", "eka"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x75, 0x72, 0x65, 0x6B, 0x61, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67,
                    0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
