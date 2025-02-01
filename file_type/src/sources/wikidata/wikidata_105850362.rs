use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850362: FileFormat = FileFormat {
    id: 105_850_362,
    puid: "wikidata/105850362",
    name: "CHX font format",
    extensions: &["chx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x58, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
