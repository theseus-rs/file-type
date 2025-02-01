use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854228: FileFormat = FileFormat {
    id: 105_854_228,
    puid: "wikidata/105854228",
    name: "CSC compressed archive",
    extensions: &["csc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x73, 0x79, 0x08])],
            },
        }],
    }],
    related_formats: &[],
};
