use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853720: FileFormat = FileFormat {
    id: 105_853_720,
    puid: "wikidata/105853720",
    name: "Abacus spreadsheet",
    extensions: &["aba"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x42, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
