use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856071: FileFormat = FileFormat {
    id: 105_856_071,
    puid: "wikidata/105856071",
    name: "OrangeCD theme",
    extensions: &["dax"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
