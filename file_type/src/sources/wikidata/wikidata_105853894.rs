use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853894: FileFormat = FileFormat {
    id: 105_853_894,
    puid: "wikidata/105853894",
    name: "Affix file",
    extensions: &["aff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x45, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
