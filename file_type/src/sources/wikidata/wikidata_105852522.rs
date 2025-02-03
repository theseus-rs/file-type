use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852522: FileFormat = FileFormat {
    id: 105_852_522,
    source_type: SourceType::Wikidata,
    name: "Aegis Animator Script",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
