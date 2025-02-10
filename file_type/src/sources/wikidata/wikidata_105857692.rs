use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857692: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_692,
        source_type: SourceType::Wikidata,
        name: "Archimedes Protected Disk image (unzipped)",
        extensions: &["adp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x50, 0x44, 0x58, 0x30, 0x30, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
