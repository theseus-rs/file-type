use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856789: FileFormat = FileFormat {
    id: 105_856_789,
    source_type: SourceType::Wikidata,
    name: "GetRight File List",
    extensions: &["grx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x52, 0x4C, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
