use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866192: FileFormat = FileFormat {
    id: 105_866_192,
    puid: "wikidata/105866192",
    name: "Windows 95 passwords",
    extensions: &["pwl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB0, 0x4D, 0x46, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
