use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_714,
        source_type: SourceType::Wikidata,
        name: "DOSIMG disk image (80t/15s)",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x0F, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
