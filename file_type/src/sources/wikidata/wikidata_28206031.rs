use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206031: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_031,
        source_type: SourceType::Wikidata,
        name: "EggPaint",
        extensions: &["trp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x52, 0x55, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
