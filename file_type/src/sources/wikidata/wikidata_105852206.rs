use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852206: FileFormat = FileFormat {
    id: 105_852_206,
    puid: "wikidata/105852206",
    name: "Cura STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["model/x.stl-binary"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x55, 0x52, 0x41, 0x20, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59, 0x20, 0x53,
                    0x54, 0x4C, 0x20, 0x45, 0x58, 0x50, 0x4F, 0x52, 0x54, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
