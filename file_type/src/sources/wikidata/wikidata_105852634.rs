use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852634: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_634,
        source_type: SourceType::Wikidata,
        name: "NGS orbital format SP3 (with velocity)",
        extensions: &["sp3"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x61, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
