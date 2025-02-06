use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867011: FileFormat = FileFormat {
    id: 105_867_011,
    source_type: SourceType::Wikidata,
    name: "Novell Groupwise Address Book",
    extensions: &["nab"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x3A, 0x3A, 0x54, 0x41, 0x47, 0x4D, 0x41, 0x50, 0x3A, 0x3A, 0x3A, 0x30,
                    0x46, 0x46, 0x45, 0x30, 0x30, 0x30, 0x33, 0x3A, 0x2A, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
