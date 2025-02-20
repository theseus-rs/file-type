use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865410: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_410,
        source_type: SourceType::Wikidata,
        name: "Premiere project",
        extensions: &["ppj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x48, 0x45, 0x41, 0x44, 0x5D, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
