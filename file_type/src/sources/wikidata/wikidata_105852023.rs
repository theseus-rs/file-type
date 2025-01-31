use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852023: FileFormat = FileFormat {
    id: 105_852_023,
    puid: "wikidata/105852023",
    name: "Stata Data format (v113, BE)",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x71, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
