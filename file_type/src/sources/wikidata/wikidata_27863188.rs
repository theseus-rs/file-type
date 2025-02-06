use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863188: FileFormat = FileFormat {
    id: 27_863_188,
    source_type: SourceType::Wikidata,
    name: "Audio Data Transport Stream",
    extensions: &["aac", "adts"],
    media_types: &["audio/vnd.dlna.adts"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xF1])],
                },
            }],
        },
        Signature {
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
