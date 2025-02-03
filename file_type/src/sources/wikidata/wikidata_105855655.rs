use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855655: FileFormat = FileFormat {
    id: 105_855_655,
    source_type: SourceType::Wikidata,
    name: "Cakewalk Overture Score",
    extensions: &["ove"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x56, 0x53, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
