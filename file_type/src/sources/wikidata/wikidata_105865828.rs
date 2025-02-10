use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865828: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_828,
        source_type: SourceType::Wikidata,
        name: "Akai MESA program",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x65, 0x73, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
