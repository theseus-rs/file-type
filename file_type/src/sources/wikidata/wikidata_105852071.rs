use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852071: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_071,
        source_type: SourceType::Wikidata,
        name: "Simplify3D STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
                        0x53, 0x69, 0x6D, 0x70, 0x6C, 0x69, 0x66, 0x79, 0x33, 0x44, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
