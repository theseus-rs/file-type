use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866055: FileFormat = FileFormat {
    id: 105_866_055,
    puid: "wikidata/105866055",
    name: "MikuMikuDance PolygonMovieMaker data",
    extensions: &["pmm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x6C, 0x79, 0x67, 0x6F, 0x6E, 0x20, 0x4D, 0x6F, 0x76, 0x69, 0x65,
                    0x20, 0x6D, 0x61, 0x6B, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
