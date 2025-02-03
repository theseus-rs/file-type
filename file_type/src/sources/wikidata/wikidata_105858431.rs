use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858431: FileFormat = FileFormat {
    id: 105_858_431,
    source_type: SourceType::Wikidata,
    name: "Endnote Export Format",
    extensions: &["enw"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x30, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
