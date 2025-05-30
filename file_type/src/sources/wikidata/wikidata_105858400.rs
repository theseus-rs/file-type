use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_400,
        source_type: SourceType::Wikidata,
        name: "QLAY MDV image",
        extensions: &["mdv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF,
                        0xFF, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
