use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851041: FileFormat = FileFormat {
    id: 105_851_041,
    puid: "wikidata/105851041",
    name: "Moebius Tile Library",
    extensions: &["tlb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4C, 0x42, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
