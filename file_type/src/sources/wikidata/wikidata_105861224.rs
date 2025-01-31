use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861224: FileFormat = FileFormat {
    id: 105_861_224,
    puid: "wikidata/105861224",
    name: "Pioneer OEL screensaver",
    extensions: &["lkd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7A, 0x4C, 0x4B, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
