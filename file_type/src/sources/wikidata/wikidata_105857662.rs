use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857662: FileFormat = FileFormat {
    id: 105_857_662,
    source_type: SourceType::Wikidata,
    name: "MIT CADR Lisp Machine disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x41, 0x42, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
