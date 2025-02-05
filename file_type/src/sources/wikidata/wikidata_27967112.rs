use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967112: FileFormat = FileFormat {
    id: 27_967_112,
    source_type: SourceType::Wikidata,
    name: "All Sound Tracker module",
    extensions: &["ast"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x08, 0x41, 0x53, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
