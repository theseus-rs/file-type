use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855000: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_000,
        source_type: SourceType::Wikidata,
        name: "ArtBorder data",
        extensions: &["artborder"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x26, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
