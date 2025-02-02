use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857193: FileFormat = FileFormat {
    id: 105_857_193,
    source_type: SourceType::Wikidata,
    name: "Turbo Pascal Help (v2)",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x48, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
