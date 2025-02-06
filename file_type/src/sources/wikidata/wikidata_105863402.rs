use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863402: FileFormat = FileFormat {
    id: 105_863_402,
    source_type: SourceType::Wikidata,
    name: "TommySoftware CAD/Draw settings (v1)",
    extensions: &["mpi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x6F, 0x6D, 0x6D, 0x79, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
                    0x20, 0x4D, 0x50, 0x49, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
