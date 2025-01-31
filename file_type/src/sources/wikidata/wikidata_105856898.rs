use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856898: FileFormat = FileFormat {
    id: 105_856_898,
    puid: "wikidata/105856898",
    name: "Geometer's Sketchpad Document",
    extensions: &["gsp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x53, 0x50, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
