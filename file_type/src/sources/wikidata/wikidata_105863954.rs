use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863954: FileFormat = FileFormat {
    id: 105_863_954,
    puid: "wikidata/105863954",
    name: "SuperTux Music",
    extensions: &["music"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x73, 0x75, 0x70, 0x65, 0x72, 0x74, 0x75, 0x78, 0x2D, 0x6D, 0x75, 0x73,
                    0x69, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
