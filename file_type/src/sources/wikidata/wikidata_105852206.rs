use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852206: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_206,
        source_type: SourceType::Wikidata,
        name: "Cura STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x55, 0x52, 0x41, 0x20, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59, 0x20,
                        0x53, 0x54, 0x4C, 0x20, 0x45, 0x58, 0x50, 0x4F, 0x52, 0x54, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
