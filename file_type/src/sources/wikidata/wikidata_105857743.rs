use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857743: FileFormat = FileFormat {
    id: 105_857_743,
    puid: "wikidata/105857743",
    name: "Ensoniq EPS EDM disk image",
    extensions: &["ede"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0x0A, 0x45, 0x50, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
