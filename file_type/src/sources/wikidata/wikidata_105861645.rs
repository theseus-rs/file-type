use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861645: FileFormat = FileFormat {
    id: 105_861_645,
    source_type: SourceType::Wikidata,
    name: "Lego Digital Designer data",
    extensions: &["lif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x49, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
