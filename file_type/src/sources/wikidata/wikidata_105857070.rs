use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857070: FileFormat = FileFormat {
    id: 105_857_070,
    source_type: SourceType::Wikidata,
    name: "Xbox Game Profile Data",
    extensions: &["gpd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x44, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
