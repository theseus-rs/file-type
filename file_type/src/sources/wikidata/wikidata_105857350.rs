use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857350: FileFormat = FileFormat {
    id: 105_857_350,
    source_type: SourceType::Wikidata,
    name: "Labeler (v3.0) / Labels Unlimited (v2.0) Job",
    extensions: &["job"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x33, 0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
