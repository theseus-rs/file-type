use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851349: FileFormat = FileFormat {
    id: 105_851_349,
    puid: "wikidata/105851349",
    name: "Binary Tiled Data File",
    extensions: &["tdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
