use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_041,
        source_type: SourceType::Wikidata,
        name: "Moebius Tile Library",
        extensions: &["tlb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4C, 0x42, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
