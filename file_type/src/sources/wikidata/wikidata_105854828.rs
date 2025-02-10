use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854828: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_828,
        source_type: SourceType::Wikidata,
        name: "YAC compressed archive",
        extensions: &["yc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x00, 0x99, 0x00, 0x00, 0x01, 0x59, 0x43, 0x00, 0x00, 0x00,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
