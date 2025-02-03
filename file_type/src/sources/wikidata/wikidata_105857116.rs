use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857116: FileFormat = FileFormat {
    id: 105_857_116,
    source_type: SourceType::Wikidata,
    name: "UCDOS Group",
    extensions: &["grp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x55, 0x4A, 0x30, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
