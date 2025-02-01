use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849303: FileFormat = FileFormat {
    id: 105_849_303,
    puid: "wikidata/105849303",
    name: "Yahoo! Widget",
    extensions: &["widget"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x64, 0x67, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
