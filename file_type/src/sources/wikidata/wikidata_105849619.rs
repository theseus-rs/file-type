use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849619: FileFormat = FileFormat {
    id: 105_849_619,
    puid: "wikidata/105849619",
    name: "Common Ground Digital Paper document",
    extensions: &["dp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x47, 0x44, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
