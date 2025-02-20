use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866462: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_462,
        source_type: SourceType::Wikidata,
        name: "Dynamic Publisher screen",
        extensions: &["pap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x20, 0x50, 0x75, 0x62, 0x6C,
                        0x69, 0x73, 0x68, 0x65, 0x72, 0x20, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
