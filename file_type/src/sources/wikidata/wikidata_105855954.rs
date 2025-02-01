use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855954: FileFormat = FileFormat {
    id: 105_855_954,
    puid: "wikidata/105855954",
    name: "Call of Duty map (generic)",
    extensions: &["d3dbsp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
