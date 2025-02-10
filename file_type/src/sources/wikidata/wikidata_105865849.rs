use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865849: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_849,
        source_type: SourceType::Wikidata,
        name: "MediaShow Production (v1.0)",
        extensions: &["prod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
