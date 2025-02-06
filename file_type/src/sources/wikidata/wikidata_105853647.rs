use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853647: FileFormat = FileFormat {
    id: 105_853_647,
    source_type: SourceType::Wikidata,
    name: "AlgoBuild project (v1.00)",
    extensions: &["algobuild"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6C, 0x67, 0x6F, 0x42, 0x75, 0x69, 0x6C, 0x64, 0x30, 0x31, 0x2E, 0x30,
                    0x30, 0x0A, 0x50, 0x4B, 0x03, 0x04,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
