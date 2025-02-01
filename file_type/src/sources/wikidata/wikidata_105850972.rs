use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850972: FileFormat = FileFormat {
    id: 105_850_972,
    puid: "wikidata/105850972",
    name: "DB/TextWorks Database Primary Textbase Definition",
    extensions: &["tba"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x42, 0x41, 0x20, 0x30, 0x32, 0x30, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
