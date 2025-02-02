use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862397: FileFormat = FileFormat {
    id: 105_862_397,
    source_type: SourceType::Wikidata,
    name: "Multimedia Builder Data",
    extensions: &["mbd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x4D, 0x4D, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
