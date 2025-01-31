use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865023: FileFormat = FileFormat {
    id: 105_865_023,
    puid: "wikidata/105865023",
    name: "Altera Pattern Capture Format",
    extensions: &["pcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
