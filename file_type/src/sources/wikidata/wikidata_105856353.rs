use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856353: FileFormat = FileFormat {
    id: 105_856_353,
    source_type: SourceType::Wikidata,
    name: "NeoRAGEx savestate",
    extensions: &["dat1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x52, 0x58, 0x53, 0x54, 0x41, 0x54, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
