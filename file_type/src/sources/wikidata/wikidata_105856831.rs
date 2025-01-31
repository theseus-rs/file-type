use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856831: FileFormat = FileFormat {
    id: 105_856_831,
    puid: "wikidata/105856831",
    name: "Glest 3D model",
    extensions: &["g3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x33, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
