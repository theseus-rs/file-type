use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866487: FileFormat = FileFormat {
    id: 105_866_487,
    puid: "wikidata/105866487",
    name: "CPython 1.x bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x99, 0x4E, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
