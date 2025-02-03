use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864133: FileFormat = FileFormat {
    id: 105_864_133,
    source_type: SourceType::Wikidata,
    name: "FM Tracker module",
    extensions: &["fmt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0x54, 0x52, 0x4B, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
