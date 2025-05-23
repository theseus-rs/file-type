use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856650: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_650,
        source_type: SourceType::Wikidata,
        name: "Wise script",
        extensions: &["wse"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x54, 0x79, 0x70,
                        0x65, 0x3A, 0x20, 0x57, 0x53, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
