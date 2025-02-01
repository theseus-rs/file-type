use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866077: FileFormat = FileFormat {
    id: 105_866_077,
    puid: "wikidata/105866077",
    name: "Pretty Good Privacy (PGP) Public Keyring",
    extensions: &["pkr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x99, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
