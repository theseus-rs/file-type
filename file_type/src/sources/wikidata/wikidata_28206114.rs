use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206114: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_114,
        source_type: SourceType::Wikidata,
        name: "Fuzzy Bitmap",
        extensions: &["cbm", "fbm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x62, 0x69, 0x74, 0x6D, 0x61, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
