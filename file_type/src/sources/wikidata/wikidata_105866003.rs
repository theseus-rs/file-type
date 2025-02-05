use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866003: FileFormat = FileFormat {
    id: 105_866_003,
    source_type: SourceType::Wikidata,
    name: "CineMorph Project",
    extensions: &["project"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x52, 0x46, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
