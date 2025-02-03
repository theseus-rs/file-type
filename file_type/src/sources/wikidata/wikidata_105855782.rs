use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855782: FileFormat = FileFormat {
    id: 105_855_782,
    source_type: SourceType::Wikidata,
    name: "Guild Wars data",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x41, 0x4E, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
