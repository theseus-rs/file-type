use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862477: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_477,
        source_type: SourceType::Wikidata,
        name: "MDIFF patch (v1.x)",
        extensions: &["mdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x46, 0x00, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
