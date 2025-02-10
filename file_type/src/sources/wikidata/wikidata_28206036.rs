use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206036: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_036,
        source_type: SourceType::Wikidata,
        name: "Enhanced Simplex",
        extensions: &["esm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4D, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
