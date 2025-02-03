use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852842: FileFormat = FileFormat {
    id: 105_852_842,
    source_type: SourceType::Wikidata,
    name: "Raw print data",
    extensions: &["spl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x49, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
