use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852267: FileFormat = FileFormat {
    id: 105_852_267,
    puid: "wikidata/105852267",
    name: "Blender STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x53, 0x54, 0x4C, 0x20, 0x6F, 0x75,
                    0x74, 0x70, 0x75, 0x74, 0x20, 0x66, 0x72, 0x6F, 0x6D, 0x20, 0x42, 0x6C, 0x65,
                    0x6E, 0x64, 0x65, 0x72, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
