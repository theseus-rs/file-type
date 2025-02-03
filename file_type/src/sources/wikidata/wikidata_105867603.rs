use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867603: FileFormat = FileFormat {
    id: 105_867_603,
    source_type: SourceType::Wikidata,
    name: "NetEase Games data Package",
    extensions: &["npk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x58, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
