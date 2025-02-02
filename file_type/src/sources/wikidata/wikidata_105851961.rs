use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851961: FileFormat = FileFormat {
    id: 105_851_961,
    source_type: SourceType::Wikidata,
    name: "Jose SciEditor Project",
    extensions: &["spf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x42, 0x41, 0x53, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x73, 0x5D, 0x0D, 0x0A,
                    0x50, 0x72, 0x69, 0x6D, 0x61, 0x72, 0x79, 0x20, 0x53, 0x6F, 0x75, 0x72, 0x63,
                    0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
