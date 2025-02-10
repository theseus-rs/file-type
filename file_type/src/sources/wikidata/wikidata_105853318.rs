use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853318: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_318,
        source_type: SourceType::Wikidata,
        name: "S20 RAM snapshot format",
        extensions: &["s20"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x49, 0x43, 0x2D, 0x32, 0x30, 0x20, 0x53, 0x4E, 0x41, 0x50, 0x53,
                        0x48, 0x4F, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
