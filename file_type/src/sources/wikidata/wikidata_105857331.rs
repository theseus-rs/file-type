use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857331: FileFormat = FileFormat {
    id: 105_857_331,
    source_type: SourceType::Wikidata,
    name: "JPC-RR rerecording",
    extensions: &["jrsr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x52, 0x53, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
