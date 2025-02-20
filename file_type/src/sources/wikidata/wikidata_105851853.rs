use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851853: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_853,
        source_type: SourceType::Wikidata,
        name: "Alibre Design STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x6C, 0x69, 0x62, 0x72, 0x65, 0x20, 0x73, 0x74, 0x6C, 0x20, 0x62,
                        0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
