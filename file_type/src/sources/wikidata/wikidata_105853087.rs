use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853087: FileFormat = FileFormat {
    id: 105_853_087,
    puid: "wikidata/105853087",
    name: "Siag spreadsheet",
    extensions: &["siag"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x73, 0x77, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
