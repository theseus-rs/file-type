use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858482: FileFormat = FileFormat {
    id: 105_858_482,
    puid: "wikidata/105858482",
    name: "Mallard BASIC tokenized source (new)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFC, 0x04, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
