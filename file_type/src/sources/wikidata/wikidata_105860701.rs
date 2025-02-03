use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860701: FileFormat = FileFormat {
    id: 105_860_701,
    source_type: SourceType::Wikidata,
    name: "Redcode source",
    extensions: &["red"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x72, 0x65, 0x64, 0x63, 0x6F, 0x64, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
