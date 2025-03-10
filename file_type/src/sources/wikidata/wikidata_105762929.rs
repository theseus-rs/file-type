use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762929: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_929,
        source_type: SourceType::Wikidata,
        name: "X-CAD Drawing",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x44, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
