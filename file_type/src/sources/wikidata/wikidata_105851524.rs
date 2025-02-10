use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851524: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_524,
        source_type: SourceType::Wikidata,
        name: "YS FLIGHT terrain data",
        extensions: &["ter"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x65, 0x72, 0x72, 0x4D, 0x65, 0x73, 0x68, 0x0D, 0x0A, 0x4E, 0x42,
                        0x4C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
