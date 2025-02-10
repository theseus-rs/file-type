use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850785: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_785,
        source_type: SourceType::Wikidata,
        name: "KbdEdit dead table",
        extensions: &["kld"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x44, 0x00, 0x45, 0x00, 0x41, 0x00, 0x44, 0x00, 0x54, 0x00,
                        0x41, 0x00, 0x42, 0x00, 0x4C, 0x00, 0x45, 0x00, 0x0D, 0x00, 0x0A, 0x00,
                        0x0D, 0x00, 0x0A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
