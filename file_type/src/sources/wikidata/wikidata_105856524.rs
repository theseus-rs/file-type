use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856524: FileFormat = FileFormat {
    id: 105_856_524,
    source_type: SourceType::Wikidata,
    name: "Kingsoft WPS Office 2001 document",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x90, 0xFF, 0x57, 0x50, 0x53, 0x32, 0x30, 0x30, 0x31, 0xAE, 0x20, 0x66, 0x6F,
                    0x72, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x4B, 0x69, 0x6E,
                    0x67, 0x73, 0x6F, 0x66, 0x74, 0xA9, 0x20, 0x32, 0x30, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
