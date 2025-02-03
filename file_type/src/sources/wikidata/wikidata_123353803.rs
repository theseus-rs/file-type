use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123353803: FileFormat = FileFormat {
    id: 123_353_803,
    source_type: SourceType::Wikidata,
    name: "C2PA Manifest",
    extensions: &["c2pa"],
    media_types: &["application/x-c2pa-manifest-store"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6A, 0x75, 0x6D, 0x64, 0x63, 0x32, 0x70, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
