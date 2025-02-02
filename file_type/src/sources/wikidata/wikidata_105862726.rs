use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862726: FileFormat = FileFormat {
    id: 105_862_726,
    source_type: SourceType::Wikidata,
    name: "Twist Mailmerge script",
    extensions: &["m"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x41, 0x49, 0x4C, 0x4D, 0x45, 0x52, 0x47, 0x45, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
