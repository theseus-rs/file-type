use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960138: FileFormat = FileFormat {
    id: 27_960_138,
    puid: "wikidata/27960138",
    name: "SPPACK",
    extensions: &["d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0xC3, 0xFC, 0x0E])],
            },
        }],
    }],
    related_formats: &[],
};
