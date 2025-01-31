use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855436: FileFormat = FileFormat {
    id: 105_855_436,
    puid: "wikidata/105855436",
    name: "GeoWorks GEOS font",
    extensions: &["fnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x53, 0x57, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
