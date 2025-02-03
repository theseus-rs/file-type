use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856663: FileFormat = FileFormat {
    id: 105_856_663,
    source_type: SourceType::Wikidata,
    name: "Crazy Machines model",
    extensions: &["ucm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x43, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
