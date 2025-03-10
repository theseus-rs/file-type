use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855370: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_370,
        source_type: SourceType::Wikidata,
        name: "Forethought PowerPoint for Mac (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0xAD, 0xDE, 0xED, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x18,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
