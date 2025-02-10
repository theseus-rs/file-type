use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207408: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_408,
        source_type: SourceType::Wikidata,
        name: "Utah RLE",
        extensions: &["rle"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0xCC])],
                },
            }],
        }],
        related_formats: &[],
    },
};
