use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863672: FileFormat = FileFormat {
    id: 105_863_672,
    puid: "wikidata/105863672",
    name: "Palm Memo Pad Archive",
    extensions: &["mpa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
