use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852018: FileFormat = FileFormat {
    id: 105_852_018,
    puid: "wikidata/105852018",
    name: "Luigi's Mansion game data",
    extensions: &["szp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x59, 0x61, 0x79, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
