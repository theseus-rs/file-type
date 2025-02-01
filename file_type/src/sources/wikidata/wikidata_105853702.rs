use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853702: FileFormat = FileFormat {
    id: 105_853_702,
    puid: "wikidata/105853702",
    name: "rzip compressed archive",
    extensions: &["rz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x5A, 0x49, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
