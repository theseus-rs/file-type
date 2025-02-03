use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867655: FileFormat = FileFormat {
    id: 105_867_655,
    source_type: SourceType::Wikidata,
    name: "Linden Notecard",
    extensions: &["notecard"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x6E, 0x64, 0x65, 0x6E, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x76,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
