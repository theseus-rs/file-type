use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862485: FileFormat = FileFormat {
    id: 105_862_485,
    source_type: SourceType::Wikidata,
    name: "MiSTer Arcade ROM configuration",
    extensions: &["mra"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x69, 0x73, 0x74, 0x65, 0x72, 0x72, 0x6F, 0x6D, 0x64, 0x65, 0x73,
                    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
