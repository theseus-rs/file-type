use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859145: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_145,
        source_type: SourceType::Wikidata,
        name: "DuneGraph Uncompressed bitmap",
        extensions: &["dgu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x47, 0x55, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
