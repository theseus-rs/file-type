use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_113,
        source_type: SourceType::Wikidata,
        name: "OME-TIFF bitmap (big endian)",
        extensions: &["tif", "tiff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
