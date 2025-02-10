use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1461901: FileFormat = FileFormat {
    id: 1_461_901,
    source_type: SourceType::Wikidata,
    name: "Scream Tracker 3 module",
    extensions: &["s3m"],
    media_types: &["audio/s3m", "audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x53, 0x43, 0x52, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
