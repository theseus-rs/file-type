use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867080: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_080,
        source_type: SourceType::Wikidata,
        name: "Nexon game archive",
        extensions: &["nar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x41, 0x52, 0x00, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
