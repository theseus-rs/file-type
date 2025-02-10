use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860306: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_306,
        source_type: SourceType::Wikidata,
        name: "Golly Extended RLE",
        extensions: &["rle"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x43, 0x58, 0x52, 0x4C, 0x45, 0x20, 0x50, 0x6F, 0x73, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
