use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856454: FileFormat = FileFormat {
    id: 105_856_454,
    source_type: SourceType::Wikidata,
    name: "WinHex backup",
    extensions: &["whx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x48, 0x58, 0x20, 0x42, 0x61, 0x63, 0x6B, 0x75, 0x70, 0x20, 0x76, 0x31,
                    0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
