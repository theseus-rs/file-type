use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854713: FileFormat = FileFormat {
    id: 105_854_713,
    puid: "wikidata/105854713",
    name: "Cartooners Actor",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x10, 0x46, 0x00, 0x00, 0x00, 0x77, 0x07, 0x85, 0x0C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
