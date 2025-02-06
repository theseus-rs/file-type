use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863698: FileFormat = FileFormat {
    id: 105_863_698,
    source_type: SourceType::Wikidata,
    name: "IceTracker module (v1.x)",
    extensions: &["it"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x54, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
