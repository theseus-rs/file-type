use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858562: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_562,
        source_type: SourceType::Wikidata,
        name: "Dynamix Bitmap data container",
        extensions: &["blk", "bmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0x50, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
