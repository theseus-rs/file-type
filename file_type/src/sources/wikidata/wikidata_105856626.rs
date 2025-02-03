use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856626: FileFormat = FileFormat {
    id: 105_856_626,
    source_type: SourceType::Wikidata,
    name: "SigmaNEST WorkSpace",
    extensions: &["ws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x07, 0x53, 0x47, 0x4D, 0x4E, 0x45, 0x53, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
