use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855362: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_362,
        source_type: SourceType::Wikidata,
        name: "IKEA Kitchen Planner Document",
        extensions: &["fpf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00,
                        0x0C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
