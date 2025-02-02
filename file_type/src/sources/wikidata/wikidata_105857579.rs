use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857579: FileFormat = FileFormat {
    id: 105_857_579,
    source_type: SourceType::Wikidata,
    name: "Actor Image snapshot (v3.1)",
    extensions: &["ima"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x10, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
