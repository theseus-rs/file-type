use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866967: FileFormat = FileFormat {
    id: 105_866_967,
    puid: "wikidata/105866967",
    name: "Neko bytecode",
    extensions: &["n"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x45, 0x4B, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
