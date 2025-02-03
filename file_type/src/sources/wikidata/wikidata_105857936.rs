use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857936: FileFormat = FileFormat {
    id: 105_857_936,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine save Game (v2.2)",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x4D, 0x45, 0x56, 0x32, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
