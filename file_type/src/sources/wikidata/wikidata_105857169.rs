use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857169: FileFormat = FileFormat {
    id: 105_857_169,
    puid: "wikidata/105857169",
    name: "OS/2 Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x50, 0x10, 0x9B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
