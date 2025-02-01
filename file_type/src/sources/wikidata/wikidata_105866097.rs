use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866097: FileFormat = FileFormat {
    id: 105_866_097,
    puid: "wikidata/105866097",
    name: "CPython 3.1 bytecode",
    extensions: &["pyc"],
    media_types: &["application/x-python-bytecode"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x0C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
