use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865235: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_235,
        source_type: SourceType::Wikidata,
        name: "PIMPLE compressed data (v1)",
        extensions: &["pim"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x50, 0x49, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
