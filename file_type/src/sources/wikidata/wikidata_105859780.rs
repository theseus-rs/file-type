use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859780: FileFormat = FileFormat {
    id: 105_859_780,
    puid: "wikidata/105859780",
    name: "Maui Runtime Environment application (Zlib packed)",
    extensions: &["vxp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0xDA])],
            },
        }],
    }],
    related_formats: &[],
};
