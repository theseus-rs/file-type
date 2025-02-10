use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858595: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_595,
        source_type: SourceType::Wikidata,
        name: "Naive Image format NIA animated bitmaps",
        extensions: &["nia"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0xC3, 0xAF, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
