use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_463,
        source_type: SourceType::Wikidata,
        name: "MINC1 Medical Imaging format",
        extensions: &["mnc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x46, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
