use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853914: FileFormat = FileFormat {
    id: 105_853_914,
    puid: "wikidata/105853914",
    name: "BriefLZ compressed data",
    extensions: &["blz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x62, 0x6C, 0x7A, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
