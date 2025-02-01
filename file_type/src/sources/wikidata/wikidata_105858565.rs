use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858565: FileFormat = FileFormat {
    id: 105_858_565,
    puid: "wikidata/105858565",
    name: "Ipix Spherical Panorama",
    extensions: &["ipx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
