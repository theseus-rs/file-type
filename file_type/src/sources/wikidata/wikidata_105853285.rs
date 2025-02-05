use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853285: FileFormat = FileFormat {
    id: 105_853_285,
    source_type: SourceType::Wikidata,
    name: "SynWrite Project",
    extensions: &["synw-proj"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x0A, 0x5B, 0x49, 0x6E, 0x69, 0x5D, 0x0A, 0x57, 0x6F, 0x72,
                    0x6B, 0x44, 0x69, 0x72, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
