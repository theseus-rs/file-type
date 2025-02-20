use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_091,
        source_type: SourceType::Wikidata,
        name: "Solid Edge STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x6F, 0x6C, 0x69, 0x64, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
                        0x20, 0x53, 0x54, 0x4C, 0x20, 0x66, 0x72, 0x6F, 0x6D, 0x20, 0x53, 0x6F,
                        0x6C, 0x69, 0x64, 0x20, 0x45, 0x64, 0x67, 0x65, 0x2C, 0x20, 0x55, 0x6E,
                        0x69, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x73, 0x20, 0x53, 0x6F,
                        0x6C, 0x75, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x20, 0x49, 0x6E, 0x63, 0x2E,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
