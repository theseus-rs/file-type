use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856205: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_205,
        source_type: SourceType::Wikidata,
        name: "X-Plane Distribution Scenery Format",
        extensions: &["dsf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x50, 0x4C, 0x4E, 0x45, 0x44, 0x53, 0x46, 0x01, 0x00, 0x00, 0x00,
                        0x44, 0x41, 0x45, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
