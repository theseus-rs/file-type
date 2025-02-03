use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849903: FileFormat = FileFormat {
    id: 105_849_903,
    source_type: SourceType::Wikidata,
    name: "DCS Campaign script",
    extensions: &["cmp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x61, 0x6D, 0x70, 0x61, 0x69, 0x67, 0x6E, 0x20, 0x3D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
