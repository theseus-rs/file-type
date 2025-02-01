use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854219: FileFormat = FileFormat {
    id: 105_854_219,
    puid: "wikidata/105854219",
    name: "AYFX Editor Bank",
    extensions: &["afb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x03, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
