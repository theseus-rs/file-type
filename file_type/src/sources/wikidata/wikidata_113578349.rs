use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113578349: FileType = FileType {
    file_format: &FileFormat {
        id: 113_578_349,
        source_type: SourceType::Wikidata,
        name: "MAGIX Video File",
        extensions: &["mxv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x58, 0x52, 0x49, 0x46, 0x46, 0x36, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
