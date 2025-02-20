use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854942: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_942,
        source_type: SourceType::Wikidata,
        name: "SAPCAR SAR compressed archive (v2.x)",
        extensions: &["sar"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x52, 0x20, 0x32, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
