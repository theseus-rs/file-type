use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856973: FileFormat = FileFormat {
    id: 105_856_973,
    puid: "wikidata/105856973",
    name: "Graphtec Binary Data",
    extensions: &["gbd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x43, 0x6F, 0x6D, 0x6D, 0x6F, 0x6E, 0x0D, 0x0A, 0x20, 0x20, 0x49, 0x44,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
