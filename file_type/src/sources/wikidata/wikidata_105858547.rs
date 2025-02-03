use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858547: FileFormat = FileFormat {
    id: 105_858_547,
    source_type: SourceType::Wikidata,
    name: "Borland Graphics printer driver (v2.x)",
    extensions: &["bgi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x53, 0x08, 0x08, 0x42, 0x47, 0x49, 0x20, 0x47, 0x72, 0x61, 0x70, 0x68,
                    0x69, 0x63, 0x73, 0x20, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x20, 0x44,
                    0x72, 0x69, 0x76, 0x65, 0x72, 0x73, 0x20, 0x56, 0x32, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
