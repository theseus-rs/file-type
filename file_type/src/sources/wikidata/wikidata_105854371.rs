use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854371: FileFormat = FileFormat {
    id: 105_854_371,
    source_type: SourceType::Wikidata,
    name: "AMOS Pro source",
    extensions: &["amos"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4F, 0x53, 0x20, 0x50, 0x72, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
