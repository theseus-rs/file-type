use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866114: FileFormat = FileFormat {
    id: 105_866_114,
    source_type: SourceType::Wikidata,
    name: "PCE block device image",
    extensions: &["pimg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x4D, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
