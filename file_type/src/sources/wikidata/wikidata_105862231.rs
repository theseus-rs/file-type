use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862231: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_231,
        source_type: SourceType::Wikidata,
        name: "Mozilla XUL FastLoad File",
        extensions: &["mfl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x50, 0x43, 0x4F, 0x4D, 0x0A, 0x4D, 0x6F, 0x7A, 0x46, 0x41, 0x53,
                        0x4C, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
