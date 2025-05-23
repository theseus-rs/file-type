use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_020,
        source_type: SourceType::Wikidata,
        name: "Crimson Fields Level data",
        extensions: &["lev"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x53, 0x4E, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
