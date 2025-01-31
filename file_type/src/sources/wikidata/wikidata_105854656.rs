use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854656: FileFormat = FileFormat {
    id: 105_854_656,
    puid: "wikidata/105854656",
    name: "PEA compressed archive (v1.x)",
    extensions: &["pea"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEA, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
