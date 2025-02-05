use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863399: FileFormat = FileFormat {
    id: 105_863_399,
    source_type: SourceType::Wikidata,
    name: "OGRE Mesh (binary)",
    extensions: &["mesh"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x10, 0x5B, 0x4D, 0x65, 0x73, 0x68, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6C,
                    0x69, 0x7A, 0x65, 0x72, 0x5F, 0x76, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
