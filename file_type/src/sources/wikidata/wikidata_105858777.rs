use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858777: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_777,
        source_type: SourceType::Wikidata,
        name: "BarTender label format",
        extensions: &["btw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x0A, 0x42, 0x61, 0x72, 0x20, 0x54, 0x65, 0x6E, 0x64, 0x65, 0x72,
                        0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x0D, 0x0A, 0x28, 0x63, 0x29, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
