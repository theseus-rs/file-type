use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865942: FileFormat = FileFormat {
    id: 105_865_942,
    puid: "wikidata/105865942",
    name: "CPython 3.5 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x16, 0x0D, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
