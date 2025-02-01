use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861732: FileFormat = FileFormat {
    id: 105_861_732,
    puid: "wikidata/105861732",
    name: "J.River Media Center plugin",
    extensions: &["mjp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
