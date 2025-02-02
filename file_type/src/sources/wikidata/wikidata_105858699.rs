use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858699: FileFormat = FileFormat {
    id: 105_858_699,
    source_type: SourceType::Wikidata,
    name: "Variant Call Format (binary) (v1)",
    extensions: &["bcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
