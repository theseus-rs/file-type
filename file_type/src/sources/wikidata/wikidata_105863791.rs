use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863791: FileFormat = FileFormat {
    id: 105_863_791,
    puid: "wikidata/105863791",
    name: "Extended M3U playlist (UTF-8)",
    extensions: &["m3u", "m3u8"],
    media_types: &["audio/x-mpegurl", "audio/x-mpegurl"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x23])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x23])],
                },
            }],
        },
    ],
    related_formats: &[],
};
