use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_585,
        source_type: SourceType::Wikidata,
        name: "Allegro data (generic)",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6C, 0x68])],
                },
            }],
        }],
        related_formats: &[],
    },
};
