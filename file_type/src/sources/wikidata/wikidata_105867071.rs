use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867071: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_071,
        source_type: SourceType::Wikidata,
        name: "Font descriptor",
        extensions: &["ntf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x31, 0x46, 0x54, 0x4E, 0x53, 0x50, 0x54, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
