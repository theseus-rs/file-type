use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857301: FileFormat = FileFormat {
    id: 105_857_301,
    source_type: SourceType::Wikidata,
    name: "Seal Help format",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x53, 0x48, 0x4C, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
