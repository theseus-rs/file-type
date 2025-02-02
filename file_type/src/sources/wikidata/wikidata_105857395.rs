use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857395: FileFormat = FileFormat {
    id: 105_857_395,
    source_type: SourceType::Wikidata,
    name: "JavaScript Bean file",
    extensions: &["jsb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x4A, 0x53, 0x42, 0x3E, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
