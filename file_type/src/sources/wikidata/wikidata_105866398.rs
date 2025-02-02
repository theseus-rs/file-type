use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866398: FileFormat = FileFormat {
    id: 105_866_398,
    source_type: SourceType::Wikidata,
    name: "MetaWare Parse Table",
    extensions: &["pt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x2D, 0x50, 0x41, 0x52, 0x53, 0x45, 0x20, 0x54, 0x41, 0x42, 0x4C,
                    0x45, 0x53, 0x2D, 0x2D, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
