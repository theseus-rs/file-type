use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862913: FileFormat = FileFormat {
    id: 105_862_913,
    puid: "wikidata/105862913",
    name: "Aegis ProMotion Motion",
    extensions: &["mot"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x4D, 0x32, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
