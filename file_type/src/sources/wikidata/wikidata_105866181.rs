use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866181: FileFormat = FileFormat {
    id: 105_866_181,
    source_type: SourceType::Wikidata,
    name: "PockEmul session",
    extensions: &["pkm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x3C, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x76, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x32, 0x2E, 0x30, 0x22, 0x20, 0x6D, 0x6F,
                    0x64, 0x65, 0x6C, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
