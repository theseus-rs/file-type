use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866841: FileFormat = FileFormat {
    id: 105_866_841,
    puid: "wikidata/105866841",
    name: "PV3D scene description data",
    extensions: &["pvd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x33, 0x44, 0x32, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
