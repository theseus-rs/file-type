use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852881: FileFormat = FileFormat {
    id: 105_852_881,
    source_type: SourceType::Wikidata,
    name: "CA-Compete! Script",
    extensions: &["scr"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72,
                    0x6F, 0x6D, 0x20, 0x43, 0x6F, 0x6D, 0x70, 0x65, 0x74, 0x65, 0x21, 0x20, 0x6D,
                    0x6F, 0x64, 0x65, 0x6C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
