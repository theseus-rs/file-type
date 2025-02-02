use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858418: FileFormat = FileFormat {
    id: 105_858_418,
    source_type: SourceType::Wikidata,
    name: "Easy CD Pro 95 Project (v2.0)",
    extensions: &["ecd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x43, 0x45, 0x69, 0x32, 0x30, 0x52, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
