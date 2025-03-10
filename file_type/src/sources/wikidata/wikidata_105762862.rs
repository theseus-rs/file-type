use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762862: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_862,
        source_type: SourceType::Wikidata,
        name: "XFBIN CPK game data container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x4B, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
