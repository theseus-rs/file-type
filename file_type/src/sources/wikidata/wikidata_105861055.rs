use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861055: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_055,
        source_type: SourceType::Wikidata,
        name: "Prima PrintMagic Layout",
        extensions: &["lyt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x50, 0x72, 0x69, 0x6D, 0x61, 0x23, 0x50, 0x72, 0x69, 0x6E, 0x74,
                        0x4D, 0x61, 0x67, 0x69, 0x63, 0x23, 0x30, 0x31, 0x2E, 0x30, 0x30, 0x2E,
                        0x30, 0x30, 0x2E, 0x30, 0x30, 0x23, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73,
                        0x79, 0x73, 0x74, 0x65, 0x6D, 0x73, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65,
                        0x65, 0x72, 0x69, 0x6E, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
