use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862524: FileFormat = FileFormat {
    id: 105_862_524,
    puid: "wikidata/105862524",
    name: "DIV Games Studio Map",
    extensions: &["map"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x61, 0x70, 0x1A, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
