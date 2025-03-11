use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860045: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_045,
        source_type: SourceType::Wikidata,
        name: "VERICUT Model file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x49, 0x43, 0x55, 0x54, 0x2D, 0x6D, 0x6F, 0x64, 0x65,
                        0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
