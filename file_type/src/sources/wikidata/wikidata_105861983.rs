use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861983: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_983,
        source_type: SourceType::Wikidata,
        name: "HP ME10 database (ASCII) (with rem)",
        extensions: &["mi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x7E, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
