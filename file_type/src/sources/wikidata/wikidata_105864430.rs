use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864430: FileFormat = FileFormat {
    id: 105_864_430,
    puid: "wikidata/105864430",
    name: "Professional Music Driver PVI samples pack (v2)",
    extensions: &["pvi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x49, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
