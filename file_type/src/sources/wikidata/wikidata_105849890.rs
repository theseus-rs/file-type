use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849890: FileFormat = FileFormat {
    id: 105_849_890,
    source_type: SourceType::Wikidata,
    name: "MS-DOS International Code Page Info",
    extensions: &["cpi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x46, 0x4F, 0x4E, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
