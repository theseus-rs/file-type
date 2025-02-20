use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851749: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_749,
        source_type: SourceType::Wikidata,
        name: "Elkulator Snapshot",
        extensions: &["snp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4C, 0x4B, 0x53, 0x4E, 0x41, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
