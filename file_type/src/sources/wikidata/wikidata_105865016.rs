use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865016: FileFormat = FileFormat {
    id: 105_865_016,
    source_type: SourceType::Wikidata,
    name: "Portable Bridge Notation (v2.0)",
    extensions: &["pbn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x20, 0x50, 0x42, 0x4E, 0x20, 0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
