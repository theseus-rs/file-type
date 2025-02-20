use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_720,
        source_type: SourceType::Wikidata,
        name: "Commodore Plus/4 BASIC V3.5 program",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
