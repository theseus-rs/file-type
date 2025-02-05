use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861874: FileFormat = FileFormat {
    id: 105_861_874,
    source_type: SourceType::Wikidata,
    name: "MediaPlayer Classic Playlist",
    extensions: &["mpcpl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x50, 0x43, 0x50, 0x4C, 0x41, 0x59, 0x4C, 0x49, 0x53, 0x54, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
