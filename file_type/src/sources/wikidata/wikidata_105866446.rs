use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866446: FileFormat = FileFormat {
    id: 105_866_446,
    source_type: SourceType::Wikidata,
    name: "CPython 2.4 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0xF2, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
