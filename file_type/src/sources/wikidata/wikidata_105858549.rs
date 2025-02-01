use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858549: FileFormat = FileFormat {
    id: 105_858_549,
    puid: "wikidata/105858549",
    name: "Mallard BASIC tokenized source (protected) (v1.11)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFC, 0x03, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
