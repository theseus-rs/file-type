use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29168314: FileFormat = FileFormat {
    id: 29_168_314,
    source_type: SourceType::Wikidata,
    name: "Microsoft Archive",
    extensions: &["mar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
