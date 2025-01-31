use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861438: FileFormat = FileFormat {
    id: 105_861_438,
    puid: "wikidata/105861438",
    name: "Actor Library Definition Language",
    extensions: &["ldl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
