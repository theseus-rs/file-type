use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855646: FileFormat = FileFormat {
    id: 105_855_646,
    puid: "wikidata/105855646",
    name: "Radiance Octree",
    extensions: &["oct"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x3F, 0x52, 0x41, 0x44, 0x49, 0x41, 0x4E, 0x43, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
