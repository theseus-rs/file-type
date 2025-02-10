use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_191,
        source_type: SourceType::Wikidata,
        name: "Dialog source code",
        extensions: &["dg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x73, 0x74, 0x6F, 0x72, 0x79, 0x20, 0x74, 0x69, 0x74, 0x6C, 0x65,
                        0x29,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
