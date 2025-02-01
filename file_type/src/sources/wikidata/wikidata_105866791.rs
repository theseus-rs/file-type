use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866791: FileFormat = FileFormat {
    id: 105_866_791,
    puid: "wikidata/105866791",
    name: "EGO Engine Textures",
    extensions: &["pssg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x53, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
