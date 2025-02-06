use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863120: FileFormat = FileFormat {
    id: 105_863_120,
    source_type: SourceType::Wikidata,
    name: "Microsoft Developer Studio Project",
    extensions: &["mdp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x53, 0x47, 0x30, 0x03, 0x00, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
