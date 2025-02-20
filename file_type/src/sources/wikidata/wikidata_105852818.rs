use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852818: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_818,
        source_type: SourceType::Wikidata,
        name: "3D Slash STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6F,
                        0x6D, 0x20, 0x33, 0x44, 0x20, 0x53, 0x6C, 0x61, 0x73, 0x68,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
