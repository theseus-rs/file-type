use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861712: FileFormat = FileFormat {
    id: 105_861_712,
    puid: "wikidata/105861712",
    name: "Dynamix Music data container",
    extensions: &["mus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
