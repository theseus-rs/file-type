use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852845: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_845,
        source_type: SourceType::Wikidata,
        name: "Roland music sequence (generic)",
        extensions: &["svq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x56, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
