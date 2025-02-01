use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863854: FileFormat = FileFormat {
    id: 105_863_854,
    puid: "wikidata/105863854",
    name: "Macrocell format",
    extensions: &["mc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4D, 0x32, 0x5D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
