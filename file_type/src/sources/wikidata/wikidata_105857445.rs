use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857445: FileFormat = FileFormat {
    id: 105_857_445,
    source_type: SourceType::Wikidata,
    name: "3D Text Commander Project",
    extensions: &["3pj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x5D, 0x0D, 0x0A, 0x46, 0x69, 0x6C, 0x65,
                    0x20, 0x6E, 0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
