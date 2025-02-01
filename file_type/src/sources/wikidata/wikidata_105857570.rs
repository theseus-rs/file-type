use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857570: FileFormat = FileFormat {
    id: 105_857_570,
    puid: "wikidata/105857570",
    name: "WinFellow emulation preset",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x5F, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
                    0x70, 0x74, 0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
