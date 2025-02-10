use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_947,
        source_type: SourceType::Wikidata,
        name: "Floppy Disk File image",
        extensions: &["fdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x46, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
