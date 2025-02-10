use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_267,
        source_type: SourceType::Wikidata,
        name: "TfID - Text File IDentifier definition (v0.x)",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x66, 0x49, 0x44, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22, 0x30,
                        0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
