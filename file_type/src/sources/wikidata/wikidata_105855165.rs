use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855165: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_165,
        source_type: SourceType::Wikidata,
        name: "ColecoVision Font",
        extensions: &["fcv"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4F, 0x4E, 0x54, 0x20, 0x43, 0x4F, 0x4C, 0x45, 0x43, 0x4F, 0x56,
                        0x49, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
