use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863963: FileFormat = FileFormat {
    id: 105_863_963,
    source_type: SourceType::Wikidata,
    name: "Moonbase Saved game",
    extensions: &["mbs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x6F, 0x6F, 0x6E, 0x42, 0x61, 0x73, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73,
                    0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30, 0x30, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
