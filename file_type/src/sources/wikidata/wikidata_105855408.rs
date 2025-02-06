use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855408: FileFormat = FileFormat {
    id: 105_855_408,
    source_type: SourceType::Wikidata,
    name: "Face The Music module",
    extensions: &["ftm"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x54, 0x4D, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
