use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855042: FileFormat = FileFormat {
    id: 105_855_042,
    puid: "wikidata/105855042",
    name: "Kexis lossless compressed audio",
    extensions: &["kxs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x45, 0x58, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
