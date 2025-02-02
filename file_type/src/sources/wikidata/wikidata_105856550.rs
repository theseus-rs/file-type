use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856550: FileFormat = FileFormat {
    id: 105_856_550,
    source_type: SourceType::Wikidata,
    name: "World of Warcraft model Skeleton",
    extensions: &["skel"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4B, 0x4C, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
