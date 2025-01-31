use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854595: FileFormat = FileFormat {
    id: 105_854_595,
    puid: "wikidata/105854595",
    name: "PEA compressed archive (v2.x)",
    extensions: &["pea"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEA, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
