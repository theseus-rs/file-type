use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859825: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_825,
        source_type: SourceType::Wikidata,
        name: "Magic Carpet Flic video",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0C, 0x00, 0x00, 0x00, 0x12, 0xAF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
