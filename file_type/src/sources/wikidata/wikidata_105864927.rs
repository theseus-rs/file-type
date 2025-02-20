use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_927,
        source_type: SourceType::Wikidata,
        name: "BIS Visitor Project",
        extensions: &["pew"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4F, 0x53, 0x45, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
