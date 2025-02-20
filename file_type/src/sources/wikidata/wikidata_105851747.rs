use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851747: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_747,
        source_type: SourceType::Wikidata,
        name: "PEPE 2D constant interval DataSet",
        extensions: &["sp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x45, 0x50, 0x45, 0x32, 0x44, 0x20, 0x63, 0x6F, 0x6E, 0x73, 0x74,
                        0x61, 0x6E, 0x74, 0x20, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6C,
                        0x20, 0x44, 0x61, 0x74, 0x61, 0x53, 0x65, 0x74, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
