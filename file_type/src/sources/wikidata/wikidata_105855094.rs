use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855094: FileFormat = FileFormat {
    id: 105_855_094,
    puid: "wikidata/105855094",
    name: "Psion Record/EPOC voice audio",
    extensions: &["prc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x7E, 0x00, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
