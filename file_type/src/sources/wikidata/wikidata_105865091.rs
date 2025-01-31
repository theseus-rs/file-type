use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865091: FileFormat = FileFormat {
    id: 105_865_091,
    puid: "wikidata/105865091",
    name: "PROTEXT document",
    extensions: &["ptx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x54, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
