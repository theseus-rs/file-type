use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866612: FileFormat = FileFormat {
    id: 105_866_612,
    puid: "wikidata/105866612",
    name: "PSXjin movie capture",
    extensions: &["pjm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4A, 0x4D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
