use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850910: FileFormat = FileFormat {
    id: 105_850_910,
    puid: "wikidata/105850910",
    name: "Doobs database",
    extensions: &["kdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD0, 0x0B])],
            },
        }],
    }],
    related_formats: &[],
};
