use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855555: FileFormat = FileFormat {
    id: 105_855_555,
    puid: "wikidata/105855555",
    name: "Oxygen Software SMS data (v3)",
    extensions: &["osm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x78, 0x79, 0x67, 0x65, 0x6E, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61,
                    0x72, 0x65, 0x20, 0x53, 0x4D, 0x53, 0x2E, 0x20, 0x76, 0x33, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
