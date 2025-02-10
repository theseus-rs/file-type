use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855634: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_634,
        source_type: SourceType::Wikidata,
        name: "Olympus digital camera RAW image (IIRS)",
        extensions: &["orf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x52, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
