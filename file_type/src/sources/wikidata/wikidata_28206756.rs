use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206756: FileFormat = FileFormat {
    id: 28_206_756,
    puid: "wikidata/28206756",
    name: "NIST IHead",
    extensions: &["pct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x38, 0x38, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
