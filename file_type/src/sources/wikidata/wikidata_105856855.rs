use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856855: FileFormat = FileFormat {
    id: 105_856_855,
    puid: "wikidata/105856855",
    name: "Mind Games - 4 in a Line saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x66, 0x6F, 0x75, 0x72, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
