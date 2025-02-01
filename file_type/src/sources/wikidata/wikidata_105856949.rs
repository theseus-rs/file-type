use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856949: FileFormat = FileFormat {
    id: 105_856_949,
    puid: "wikidata/105856949",
    name: "Surfer Grid",
    extensions: &["grd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x41, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
