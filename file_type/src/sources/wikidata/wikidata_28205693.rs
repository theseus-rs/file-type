use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205693: FileFormat = FileFormat {
    id: 28_205_693,
    source_type: SourceType::Wikidata,
    name: "Analyze AVW",
    extensions: &["avw"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x56, 0x57, 0x5F, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
