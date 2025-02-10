use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_110501909: FileType = FileType {
    file_format: &FileFormat {
        id: 110_501_909,
        source_type: SourceType::Wikidata,
        name: "ColdFusion Markup Language format",
        extensions: &["cfm"],
        media_types: &["application/x-coldfusion"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x63, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
