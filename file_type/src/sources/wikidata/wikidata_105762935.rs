use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762935: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_935,
        source_type: SourceType::Wikidata,
        name: "blueMSX theme",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x62, 0x6C, 0x75, 0x65, 0x6D, 0x73, 0x78, 0x74, 0x68, 0x65, 0x6D,
                        0x65, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
