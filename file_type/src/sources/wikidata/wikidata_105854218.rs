use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854218: FileFormat = FileFormat {
    id: 105_854_218,
    source_type: SourceType::Wikidata,
    name: "Advanced Input Recording",
    extensions: &["air"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x70, 0x65, 0x6E, 0x41, 0x69, 0x72, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
