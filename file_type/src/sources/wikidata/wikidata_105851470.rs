use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851470: FileFormat = FileFormat {
    id: 105_851_470,
    source_type: SourceType::Wikidata,
    name: "TexFont",
    extensions: &["txf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x74, 0x78, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
