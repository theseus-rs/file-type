use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857121: FileFormat = FileFormat {
    id: 105_857_121,
    source_type: SourceType::Wikidata,
    name: "GS Draw drawing",
    extensions: &["grf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x53, 0x20, 0x44, 0x72, 0x61, 0x77, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x3A,
                    0x20, 0x20, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x47,
                    0x6F, 0x6C, 0x64, 0x65, 0x6E, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72,
                    0x65, 0x20, 0x49, 0x6E, 0x63, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
