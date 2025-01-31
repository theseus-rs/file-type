use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852749: FileFormat = FileFormat {
    id: 105_852_749,
    puid: "wikidata/105852749",
    name: "Stata Data format (v113, LE)",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x71, 0x02, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
