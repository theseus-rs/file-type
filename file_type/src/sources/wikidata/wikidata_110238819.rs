use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238819: FileFormat = FileFormat {
    id: 110_238_819,
    source_type: SourceType::Wikidata,
    name: "Movie Magic Scheduling Export",
    extensions: &["sex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x53, 0x49, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
