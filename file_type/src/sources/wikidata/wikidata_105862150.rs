use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_150,
        source_type: SourceType::Wikidata,
        name: "Marcel document",
        extensions: &[],
        media_types: &["application/rtf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x5C, 0x72, 0x74, 0x66, 0x31, 0x5C, 0x61, 0x6E, 0x73, 0x69, 0x20,
                        0x7B, 0x5C, 0x2A, 0x5C, 0x6D, 0x61, 0x72, 0x63, 0x65, 0x6C, 0x20, 0x4D,
                        0x61, 0x72, 0x63, 0x65, 0x6C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
