use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865307: FileFormat = FileFormat {
    id: 105_865_307,
    source_type: SourceType::Wikidata,
    name: "MS Visual C++ precompiled header",
    extensions: &["pch"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x43, 0x50, 0x43, 0x48, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
