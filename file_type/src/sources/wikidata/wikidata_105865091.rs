use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_091,
        source_type: SourceType::Wikidata,
        name: "PROTEXT document",
        extensions: &["ptx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x54, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
