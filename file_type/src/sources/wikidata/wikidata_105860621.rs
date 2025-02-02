use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860621: FileFormat = FileFormat {
    id: 105_860_621,
    source_type: SourceType::Wikidata,
    name: "RAGE Package Format (GTA V)",
    extensions: &["rpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x37, 0x46, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
