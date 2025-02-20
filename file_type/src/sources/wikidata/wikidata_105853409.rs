use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853409: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_409,
        source_type: SourceType::Wikidata,
        name: "Standard Test and Programming Language",
        extensions: &["stapl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x4F, 0x54, 0x45, 0x20, 0x22, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4F,
                        0x52, 0x22, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
