use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866113: FileFormat = FileFormat {
    id: 105_866_113,
    source_type: SourceType::Wikidata,
    name: "Kodak Precision Transform",
    extensions: &["pt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x75, 0x74, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
