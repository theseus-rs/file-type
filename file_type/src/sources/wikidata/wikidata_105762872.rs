use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762872: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_872,
        source_type: SourceType::Wikidata,
        name: "Compiled X KeyMap (LSB)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x6B, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
