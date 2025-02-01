use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861293: FileFormat = FileFormat {
    id: 105_861_293,
    puid: "wikidata/105861293",
    name: "Jeppesen/Mentor FliteLog Log",
    extensions: &["lbk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC8, 0x00, 0x79, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
