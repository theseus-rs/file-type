use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856935: FileFormat = FileFormat {
    id: 105_856_935,
    puid: "wikidata/105856935",
    name: "GLF 3D Font File Format",
    extensions: &["glf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4C, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
