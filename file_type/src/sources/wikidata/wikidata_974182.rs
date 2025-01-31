use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_974182: FileFormat = FileFormat {
    id: 974_182,
    puid: "wikidata/974182",
    name: "Universal 3D",
    extensions: &["u3d"],
    media_types: &["model/u3d"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x33, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
