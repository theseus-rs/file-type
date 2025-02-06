use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857679: FileFormat = FileFormat {
    id: 105_857_679,
    source_type: SourceType::Wikidata,
    name: "Opus Creator multimedia file",
    extensions: &["ilm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0xE0, 0x97, 0x2C, 0xC6, 0x26, 0x4A, 0x4F, 0xB8, 0x84, 0x56, 0xD1, 0x9C,
                    0x68, 0xD2, 0xE3, 0x69, 0x6C, 0x6D, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
