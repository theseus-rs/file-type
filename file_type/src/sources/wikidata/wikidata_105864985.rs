use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864985: FileFormat = FileFormat {
    id: 105_864_985,
    source_type: SourceType::Wikidata,
    name: "CPython 3.4 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEE, 0x0C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
