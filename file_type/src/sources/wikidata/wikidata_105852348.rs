use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852348: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_348,
        source_type: SourceType::Wikidata,
        name: "SketchUp STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6B, 0x65, 0x74, 0x63, 0x68, 0x55, 0x70, 0x20, 0x53, 0x54, 0x4C,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
