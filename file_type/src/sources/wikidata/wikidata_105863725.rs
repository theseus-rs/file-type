use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863725: FileFormat = FileFormat {
    id: 105_863_725,
    source_type: SourceType::Wikidata,
    name: "LucasArts Ad Lib Audio module",
    extensions: &["laa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
