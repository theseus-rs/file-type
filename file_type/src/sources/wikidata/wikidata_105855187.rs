use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855187: FileFormat = FileFormat {
    id: 105_855_187,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts graphics",
    extensions: &["fsh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x68, 0x70, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
