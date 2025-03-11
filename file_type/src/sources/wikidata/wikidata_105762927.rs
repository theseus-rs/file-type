use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_927,
        source_type: SourceType::Wikidata,
        name: "TEI document (UTF-8)",
        extensions: &[],
        media_types: &["application/tei+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
