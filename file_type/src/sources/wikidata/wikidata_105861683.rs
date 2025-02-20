use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861683: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_683,
        source_type: SourceType::Wikidata,
        name: "Mophun game",
        extensions: &["mpn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4D, 0x47, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
