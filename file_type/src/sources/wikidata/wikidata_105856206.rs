use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856206: FileFormat = FileFormat {
    id: 105_856_206,
    puid: "wikidata/105856206",
    name: "DST compressed file",
    extensions: &["dst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x54, 0x62])],
            },
        }],
    }],
    related_formats: &[],
};
