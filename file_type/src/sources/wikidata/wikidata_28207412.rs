use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207412: FileFormat = FileFormat {
    id: 28_207_412,
    puid: "wikidata/28207412",
    name: "Valve Texture Format",
    extensions: &["vtf"],
    media_types: &["image/vnd.valve.source.texture"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x54, 0x46, 0x00, 0x07, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
