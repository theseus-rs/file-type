use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854923: FileFormat = FileFormat {
    id: 105_854_923,
    puid: "wikidata/105854923",
    name: "Siva archive (v1)",
    extensions: &["siva"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x41, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
