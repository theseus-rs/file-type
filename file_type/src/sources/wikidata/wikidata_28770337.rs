use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770337: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_337,
        source_type: SourceType::Wikidata,
        name: "lrz",
        extensions: &["lrz"],
        media_types: &["application/x-lrzip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x52, 0x5A, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
