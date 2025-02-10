use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852986: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_986,
        source_type: SourceType::Wikidata,
        name: "Spruce subtitles (with rem)",
        extensions: &["stl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
