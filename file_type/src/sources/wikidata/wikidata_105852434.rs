use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852434: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_434,
        source_type: SourceType::Wikidata,
        name: "SubRip subtitles (UTF-16)",
        extensions: &["srt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFE, 0x31, 0x00, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
