use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853237: FileFormat = FileFormat {
    id: 105_853_237,
    puid: "wikidata/105853237",
    name: "X-CAD SoftFont",
    extensions: &["sft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
