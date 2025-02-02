use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860209: FileFormat = FileFormat {
    id: 105_860_209,
    source_type: SourceType::Wikidata,
    name: "AskEnv Requester definition",
    extensions: &["req"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
