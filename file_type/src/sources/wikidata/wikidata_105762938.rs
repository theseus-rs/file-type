use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762938: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_938,
        source_type: SourceType::Wikidata,
        name: "KiriKiri Adventure Game System package",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x50, 0x33, 0x0D, 0x0A, 0x20, 0x0A, 0x1A, 0x8B, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
