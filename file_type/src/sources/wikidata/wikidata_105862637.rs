use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862637: FileFormat = FileFormat {
    id: 105_862_637,
    puid: "wikidata/105862637",
    name: "Mediatek Font",
    extensions: &["mtf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4F, 0x4E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
