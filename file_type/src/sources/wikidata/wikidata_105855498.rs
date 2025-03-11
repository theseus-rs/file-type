use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855498: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_498,
        source_type: SourceType::Wikidata,
        name: "File Imploder compressed data (clone 7)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x2E, 0x48, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
