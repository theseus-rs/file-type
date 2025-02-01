use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851113: FileFormat = FileFormat {
    id: 105_851_113,
    puid: "wikidata/105851113",
    name: "TSD encrypted data",
    extensions: &["tsd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x54, 0x53, 0x44, 0x2D, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2D, 0x23,
                    0x23, 0x23, 0x25,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
