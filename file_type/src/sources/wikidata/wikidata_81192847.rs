use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_81192847: FileFormat = FileFormat {
    id: 81_192_847,
    source_type: SourceType::Wikidata,
    name: "The Bee Archiver compressed archive",
    extensions: &["bee"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
