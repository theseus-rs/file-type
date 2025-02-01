use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855297: FileFormat = FileFormat {
    id: 105_855_297,
    puid: "wikidata/105855297",
    name: "StarWriter for MS-DOS Formula",
    extensions: &["frm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4A, 0x01, 0x00, 0x05, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
