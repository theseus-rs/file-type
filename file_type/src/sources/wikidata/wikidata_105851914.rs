use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851914: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_914,
        source_type: SourceType::Wikidata,
        name: "StarCraft 2 Map Header",
        extensions: &["s2mh"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x6D, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22,
                        0x3A, 0x31, 0x32, 0x35, 0x37, 0x36, 0x2C, 0x22, 0x6D, 0x5F, 0x62, 0x6E,
                        0x65, 0x74, 0x22, 0x3A, 0x7B, 0x22, 0x6D, 0x5F, 0x68, 0x61, 0x6E, 0x64,
                        0x6C, 0x65, 0x22, 0x3A, 0x7B, 0x22, 0x6D, 0x5F, 0x69, 0x64, 0x22, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
