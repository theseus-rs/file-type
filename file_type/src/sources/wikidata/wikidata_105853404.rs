use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853404: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_404,
        source_type: SourceType::Wikidata,
        name: "CloudCompare STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x53, 0x54, 0x4C, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65,
                        0x64, 0x20, 0x62, 0x79, 0x20, 0x43, 0x6C, 0x6F, 0x75, 0x64, 0x43, 0x6F,
                        0x6D, 0x70, 0x61, 0x72, 0x65, 0x21, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
