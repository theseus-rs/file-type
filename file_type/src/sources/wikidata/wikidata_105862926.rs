use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862926: FileFormat = FileFormat {
    id: 105_862_926,
    puid: "wikidata/105862926",
    name: "Norton pcAnywhere Modem list",
    extensions: &["md6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x6F, 0x72, 0x74, 0x6F, 0x6E, 0x20, 0x70, 0x63, 0x41, 0x4E, 0x59, 0x57,
                    0x48, 0x45, 0x52, 0x45, 0x20, 0x4D, 0x6F, 0x64, 0x65, 0x6D, 0x20, 0x4C, 0x69,
                    0x73, 0x74, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
