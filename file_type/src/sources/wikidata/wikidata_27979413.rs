use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979413: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_413,
        source_type: SourceType::Wikidata,
        name: "TUNDRA",
        extensions: &["tnd"],
        media_types: &["application/octet-stream", "text/x-tundra"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x18, 0x54, 0x55, 0x4E, 0x44, 0x52, 0x41, 0x32, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
