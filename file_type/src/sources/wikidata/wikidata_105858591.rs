use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858591: FileFormat = FileFormat {
    id: 105_858_591,
    source_type: SourceType::Wikidata,
    name: "Funpaint 2 bitmap",
    extensions: &["fp2", "fun"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xF0, 0x3F, 0x46, 0x55, 0x4E, 0x50, 0x41, 0x49, 0x4E, 0x54, 0x20, 0x28, 0x4D,
                    0x54, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
