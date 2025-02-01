use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855028: FileFormat = FileFormat {
    id: 105_855_028,
    puid: "wikidata/105855028",
    name: "Dalet Sound format audio (old)",
    extensions: &["snd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE3, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
