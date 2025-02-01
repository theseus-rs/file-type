use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850364: FileFormat = FileFormat {
    id: 105_850_364,
    puid: "wikidata/105850364",
    name: "Canon Design Essentials printer info",
    extensions: &["csc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x63, 0x73, 0x63, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
