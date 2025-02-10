use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856064: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_064,
        source_type: SourceType::Wikidata,
        name: "Word Microsoft Office Open XML Format document (with Macro)",
        extensions: &["docm"],
        media_types: &["application/vnd.ms-word.document.macroEnabled.12"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
