use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865131: FileFormat = FileFormat {
    id: 105_865_131,
    puid: "wikidata/105865131",
    name: "CPython 2.2 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0xED, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
