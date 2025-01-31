use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861976: FileFormat = FileFormat {
    id: 105_861_976,
    puid: "wikidata/105861976",
    name: "Mobirise project",
    extensions: &["mobirise"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x0A, 0x20, 0x20, 0x22, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73,
                    0x22, 0x3A, 0x20, 0x7B, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
