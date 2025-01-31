use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853063: FileFormat = FileFormat {
    id: 105_853_063,
    puid: "wikidata/105853063",
    name: "Super Data Format",
    extensions: &["sdf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x41, 0x42, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
