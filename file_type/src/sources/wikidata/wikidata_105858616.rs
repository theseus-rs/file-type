use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_616,
        source_type: SourceType::Wikidata,
        name: "Advanced Image Coding bitmap",
        extensions: &["aic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x49, 0x43, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
