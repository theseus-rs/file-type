use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_122,
        source_type: SourceType::Wikidata,
        name: "ROFF 3D animation",
        extensions: &["rof"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x4F, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
