use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854849: FileFormat = FileFormat {
    id: 105_854_849,
    puid: "wikidata/105854849",
    name: "Generic Header audio stream",
    extensions: &["genh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x45, 0x4E, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
