use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855749: FileFormat = FileFormat {
    id: 105_855_749,
    puid: "wikidata/105855749",
    name: "Digital-FM module",
    extensions: &["dfm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x46, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
