use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863474: FileFormat = FileFormat {
    id: 105_863_474,
    source_type: SourceType::Wikidata,
    name: "MioMotion movie",
    extensions: &["mio"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x43, 0x4D, 0x4F, 0x42, 0x4A, 0x45, 0x43, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
