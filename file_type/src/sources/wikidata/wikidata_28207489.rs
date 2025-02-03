use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207489: FileFormat = FileFormat {
    id: 28_207_489,
    source_type: SourceType::Wikidata,
    name: "WhyPic",
    extensions: &["ypc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
