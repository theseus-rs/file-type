use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859403: FileFormat = FileFormat {
    id: 105_859_403,
    source_type: SourceType::Wikidata,
    name: "Q Light Controller+ Midi template",
    extensions: &["qxm"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4D, 0x69, 0x64,
                    0x69, 0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74, 0x65, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
