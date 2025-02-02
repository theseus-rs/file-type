use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864334: FileFormat = FileFormat {
    id: 105_864_334,
    source_type: SourceType::Wikidata,
    name: "The Need for Speed car Performance Specs",
    extensions: &["pbs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0xFB, 0x00, 0x07, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
