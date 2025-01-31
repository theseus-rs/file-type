use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865266: FileFormat = FileFormat {
    id: 105_865_266,
    puid: "wikidata/105865266",
    name: "CPython 3.2 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x0C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
