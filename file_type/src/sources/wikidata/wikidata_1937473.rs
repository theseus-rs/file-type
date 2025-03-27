use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1937473: FileType = FileType {
    file_format: &FileFormat {
        id: 1_937_473,
        source_type: SourceType::Wikidata,
        name: "Minolta RAW",
        extensions: &["mrw"],
        media_types: &["image/x-minolta-mrw", "image/x-raw-minolta"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x4D, 0x52, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
