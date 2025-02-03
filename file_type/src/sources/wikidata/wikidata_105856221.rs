use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856221: FileFormat = FileFormat {
    id: 105_856_221,
    source_type: SourceType::Wikidata,
    name: "Digital Sound Interface Kit module",
    extensions: &["dsm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x4D, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
