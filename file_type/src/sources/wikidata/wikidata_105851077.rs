use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851077: FileFormat = FileFormat {
    id: 105_851_077,
    source_type: SourceType::Wikidata,
    name: "SlamDB Database v1.0",
    extensions: &["tdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x44, 0x42, 0x46, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
