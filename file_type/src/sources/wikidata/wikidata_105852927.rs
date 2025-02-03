use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852927: FileFormat = FileFormat {
    id: 105_852_927,
    source_type: SourceType::Wikidata,
    name: "Creature House Expression Skeletal Stroke",
    extensions: &["sks"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x53, 0x4B, 0x33, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
