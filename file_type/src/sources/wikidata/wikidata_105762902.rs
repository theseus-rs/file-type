use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_902,
        source_type: SourceType::Wikidata,
        name: "Autodesk External Message compiled",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x64, 0x65, 0x73, 0x6B, 0x20, 0x45, 0x78, 0x74,
                        0x65, 0x72, 0x6E, 0x61, 0x6C, 0x20, 0x4D, 0x65, 0x73, 0x73, 0x61, 0x67,
                        0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
