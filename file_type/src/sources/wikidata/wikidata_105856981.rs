use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856981: FileFormat = FileFormat {
    id: 105_856_981,
    puid: "wikidata/105856981",
    name: "Genstat Book",
    extensions: &["gwb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x57, 0x42, 0x01, 0x44, 0x42, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
