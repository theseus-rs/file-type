use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863576: FileFormat = FileFormat {
    id: 105_863_576,
    source_type: SourceType::Wikidata,
    name: "The Colony level map",
    extensions: &["1"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x56, 0x45, 0x00, 0x01, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
