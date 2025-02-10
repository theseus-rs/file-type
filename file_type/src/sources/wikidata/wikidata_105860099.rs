use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860099: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_099,
        source_type: SourceType::Wikidata,
        name: "FontLab Font",
        extensions: &["vfb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x57, 0x4C, 0x46, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
