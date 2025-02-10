use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856051: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_051,
        source_type: SourceType::Wikidata,
        name: "Oric disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x52, 0x49, 0x43, 0x44, 0x49, 0x53, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
