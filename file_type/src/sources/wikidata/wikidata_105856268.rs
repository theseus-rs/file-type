use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856268: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_268,
        source_type: SourceType::Wikidata,
        name: "Dr. Web Language data",
        extensions: &["dwl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x72, 0x2E, 0x20, 0x57, 0x65, 0x62, 0x20, 0x33, 0x32, 0x20, 0x62,
                        0x69, 0x74, 0x20, 0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20,
                        0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
