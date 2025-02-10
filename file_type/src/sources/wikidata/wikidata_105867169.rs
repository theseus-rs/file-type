use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867169: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_169,
        source_type: SourceType::Wikidata,
        name: "NIfTI-2 data format (big endian)",
        extensions: &["nii"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x1C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
