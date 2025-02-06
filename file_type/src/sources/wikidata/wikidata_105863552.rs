use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863552: FileFormat = FileFormat {
    id: 105_863_552,
    source_type: SourceType::Wikidata,
    name: "SynTracker module",
    extensions: &["synmod"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x4E, 0x54, 0x52, 0x41, 0x43, 0x4B, 0x45, 0x52, 0x2D, 0x53, 0x4F,
                    0x4E, 0x47, 0x3A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
