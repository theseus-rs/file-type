use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72198767: FileFormat = FileFormat {
    id: 72_198_767,
    source_type: SourceType::Wikidata,
    name: "dBASE compiled Label file format",
    extensions: &["lbo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x44, 0x42, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
