use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770337: FileFormat = FileFormat {
    id: 28_770_337,
    puid: "wikidata/28770337",
    name: "lrz",
    extensions: &["lrz"],
    media_types: &["application/x-lrzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x52, 0x5A, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
