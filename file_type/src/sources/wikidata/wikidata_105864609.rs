use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864609: FileFormat = FileFormat {
    id: 105_864_609,
    source_type: SourceType::Wikidata,
    name: "Psion Physical Device Driver",
    extensions: &["pdd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x44, 0x44, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x79, 0x70, 0x65, 0x2A, 0x2A,
                    0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
