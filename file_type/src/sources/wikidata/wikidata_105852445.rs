use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852445: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_445,
        source_type: SourceType::Wikidata,
        name: "Sxz hybrid vector/raster image",
        extensions: &["sxz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x58, 0x5A, 0x46, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
