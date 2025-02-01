use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857633: FileFormat = FileFormat {
    id: 105_857_633,
    puid: "wikidata/105857633",
    name: "QDOS QL floppy disk image (DS/DD)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4C, 0x35, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
