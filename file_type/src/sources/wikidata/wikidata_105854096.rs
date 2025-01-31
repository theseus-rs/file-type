use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854096: FileFormat = FileFormat {
    id: 105_854_096,
    puid: "wikidata/105854096",
    name: "ARQ archive",
    extensions: &["arq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x57, 0x04, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
