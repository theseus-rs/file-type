use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851430: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_430,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior Target Data (Big Endian)",
        extensions: &["tdt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x6F, 0x6F, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
