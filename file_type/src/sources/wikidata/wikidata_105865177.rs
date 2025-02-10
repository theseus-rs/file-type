use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865177: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_177,
        source_type: SourceType::Wikidata,
        name: "Commodore 65 BASIC V10.0 program",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
