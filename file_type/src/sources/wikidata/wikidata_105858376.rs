use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858376: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_376,
        source_type: SourceType::Wikidata,
        name: "EnCase data (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x6E, 0x46, 0x09, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
