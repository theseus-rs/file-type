use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854928: FileFormat = FileFormat {
    id: 105_854_928,
    source_type: SourceType::Wikidata,
    name: "Apple II Oasis for Windows savestate",
    extensions: &["a4w"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x41, 0x70, 0x70, 0x6C, 0x65, 0x20, 0x49, 0x49, 0x5D, 0x0D, 0x0A, 0x54,
                    0x79, 0x70, 0x65, 0x3D, 0x53, 0x61, 0x76, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65,
                    0x0D, 0x0A, 0x1A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
