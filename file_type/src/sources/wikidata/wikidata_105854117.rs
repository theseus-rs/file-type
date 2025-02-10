use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_117,
        source_type: SourceType::Wikidata,
        name: "Monkey's Audio Image Link file",
        extensions: &["apl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4D, 0x6F, 0x6E, 0x6B, 0x65, 0x79, 0x27, 0x73, 0x20, 0x41, 0x75,
                        0x64, 0x69, 0x6F, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x4C, 0x69,
                        0x6E, 0x6B, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
