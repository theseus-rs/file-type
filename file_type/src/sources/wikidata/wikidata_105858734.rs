use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858734: FileFormat = FileFormat {
    id: 105_858_734,
    puid: "wikidata/105858734",
    name: "Boom Box project (v1)",
    extensions: &["box"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x58, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
