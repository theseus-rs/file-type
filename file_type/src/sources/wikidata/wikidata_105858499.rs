use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858499: FileFormat = FileFormat {
    id: 105_858_499,
    source_type: SourceType::Wikidata,
    name: "Silicon Graphics B/W bitmap",
    extensions: &["bw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0xDA, 0x01, 0x01, 0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
