use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_242,
        source_type: SourceType::Wikidata,
        name: "OS/2 Device Driver Profile",
        extensions: &["ddp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
