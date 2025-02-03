use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862874: FileFormat = FileFormat {
    id: 105_862_874,
    source_type: SourceType::Wikidata,
    name: "SNSF Super Nintendo Sound Format rip (mini)",
    extensions: &["minisnsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x46, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
