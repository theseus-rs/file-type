use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854083: FileFormat = FileFormat {
    id: 105_854_083,
    puid: "wikidata/105854083",
    name: "DLT game data archive",
    extensions: &["dlt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x56, 0x45, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
