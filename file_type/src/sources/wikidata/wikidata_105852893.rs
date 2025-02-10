use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_893,
        source_type: SourceType::Wikidata,
        name: "Uranium STLWriter STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["model/x.stl-binary"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x72, 0x61, 0x6E, 0x69, 0x75, 0x6D, 0x20, 0x53, 0x54, 0x4C, 0x57,
                        0x72, 0x69, 0x74, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
