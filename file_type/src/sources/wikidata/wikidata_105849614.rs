use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_614,
        source_type: SourceType::Wikidata,
        name: "BGI (Borland Graphics Interface) font",
        extensions: &["chr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4B, 0x08, 0x08, 0x42, 0x47, 0x49, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
