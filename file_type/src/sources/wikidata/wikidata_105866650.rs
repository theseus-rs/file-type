use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866650: FileFormat = FileFormat {
    id: 105_866_650,
    puid: "wikidata/105866650",
    name: "Professional Music Driver PZI samples pack (v1)",
    extensions: &["pzi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x5A, 0x49, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
