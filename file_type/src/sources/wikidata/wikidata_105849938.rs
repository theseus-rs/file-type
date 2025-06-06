use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849938: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_938,
        source_type: SourceType::Wikidata,
        name: "KiCad footprint information",
        extensions: &["cmp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6D, 0x70, 0x2D, 0x4D, 0x6F, 0x64, 0x20, 0x56, 0x30, 0x31, 0x20,
                        0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
