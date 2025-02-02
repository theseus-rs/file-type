use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858581: FileFormat = FileFormat {
    id: 105_858_581,
    source_type: SourceType::Wikidata,
    name: "ArtMaster88 bitmap",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x53, 0x5F, 0x53, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
