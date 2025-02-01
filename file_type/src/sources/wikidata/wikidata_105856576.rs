use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856576: FileFormat = FileFormat {
    id: 105_856_576,
    puid: "wikidata/105856576",
    name: "The Witcher 2 Entity",
    extensions: &["w2ent"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x32, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
