use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862012: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_012,
        source_type: SourceType::Wikidata,
        name: "SunVox module",
        extensions: &["sunvox"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x56, 0x4F, 0x58, 0x00, 0x00, 0x00, 0x00, 0x56, 0x45, 0x52, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
