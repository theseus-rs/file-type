use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858813: FileFormat = FileFormat {
    id: 105_858_813,
    source_type: SourceType::Wikidata,
    name: "Bochs configuration",
    extensions: &["bxrc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
                    0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72,
                    0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x42, 0x6F, 0x63, 0x68, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
