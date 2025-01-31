use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858197: FileFormat = FileFormat {
    id: 105_858_197,
    puid: "wikidata/105858197",
    name: "egis encrypted data",
    extensions: &["egisenc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0xD6, 0xEB, 0xF8])],
            },
        }],
    }],
    related_formats: &[],
};
