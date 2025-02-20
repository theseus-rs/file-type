use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_179,
        source_type: SourceType::Wikidata,
        name: "TomTom info (generic)",
        extensions: &["bif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x54, 0x6F, 0x6D, 0x54, 0x6F, 0x6D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
