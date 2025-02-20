use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861350: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_350,
        source_type: SourceType::Wikidata,
        name: "Altium Designer Layer Pairs export data",
        extensions: &["ldp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x79, 0x65, 0x72, 0x20, 0x50, 0x61, 0x69, 0x72, 0x73, 0x20,
                        0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x66, 0x6F, 0x72, 0x20, 0x50, 0x43, 0x42, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
