use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854734: FileFormat = FileFormat {
    id: 105_854_734,
    source_type: SourceType::Wikidata,
    name: "Anim8or project",
    extensions: &["an8"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
