use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854586: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_586,
        source_type: SourceType::Wikidata,
        name: "Motion Analysis Corp. ANC format",
        extensions: &["anc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x5F, 0x54, 0x79, 0x70, 0x65, 0x3A, 0x09, 0x41,
                        0x6E, 0x61, 0x6C, 0x6F, 0x67, 0x20, 0x52, 0x2F, 0x43, 0x20, 0x41, 0x53,
                        0x43, 0x49, 0x49, 0x09, 0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x69,
                        0x6F, 0x6E, 0x23, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
