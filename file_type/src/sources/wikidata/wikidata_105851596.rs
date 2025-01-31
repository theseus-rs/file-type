use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851596: FileFormat = FileFormat {
    id: 105_851_596,
    puid: "wikidata/105851596",
    name: "Skyland's Star game data",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3F, 0x63, 0x77, 0x31, 0x20, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
