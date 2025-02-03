use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858634: FileFormat = FileFormat {
    id: 105_858_634,
    source_type: SourceType::Wikidata,
    name: "DB2 Bind (old)",
    extensions: &["bnd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFB, 0x49, 0x4E, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
