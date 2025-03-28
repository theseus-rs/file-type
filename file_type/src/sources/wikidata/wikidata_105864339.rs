use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864339: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_339,
        source_type: SourceType::Wikidata,
        name: "Wwise sound archive",
        extensions: &["pck"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4B, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
