use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856462: FileFormat = FileFormat {
    id: 105_856_462,
    puid: "wikidata/105856462",
    name: "PGE World Map",
    extensions: &["wldx"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
