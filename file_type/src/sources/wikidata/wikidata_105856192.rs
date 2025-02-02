use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856192: FileFormat = FileFormat {
    id: 105_856_192,
    source_type: SourceType::Wikidata,
    name: "TOPO topographic Data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4F, 0x50, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
