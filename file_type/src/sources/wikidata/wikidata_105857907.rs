use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857907: FileFormat = FileFormat {
    id: 105_857_907,
    source_type: SourceType::Wikidata,
    name: "VICE settings",
    extensions: &["ini"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x43, 0x36, 0x34, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
