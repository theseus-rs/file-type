use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856276: FileFormat = FileFormat {
    id: 105_856_276,
    source_type: SourceType::Wikidata,
    name: "DAUB drawing (v2.x)",
    extensions: &["dob"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x44, 0x42, 0x42, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
