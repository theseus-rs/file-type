use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857098: FileFormat = FileFormat {
    id: 105_857_098,
    puid: "wikidata/105857098",
    name: "Storage card file segments Tiger Tree Hash",
    extensions: &["gltth"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x6C, 0x2B, 0x2B])],
            },
        }],
    }],
    related_formats: &[],
};
