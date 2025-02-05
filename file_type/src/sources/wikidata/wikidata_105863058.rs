use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863058: FileFormat = FileFormat {
    id: 105_863_058,
    source_type: SourceType::Wikidata,
    name: "MVSTracker Music module",
    extensions: &["mus"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x56, 0x53, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
