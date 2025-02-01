use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863188: FileFormat = FileFormat {
    id: 27_863_188,
    puid: "wikidata/27863188",
    name: "Audio Data Transport Stream",
    extensions: &["aac", "adts"],
    media_types: &["audio/vnd.dlna.adts", "audio/vnd.dlna.adts"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xF1])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xF1])],
                },
            }],
        },
    ],
    related_formats: &[],
};
