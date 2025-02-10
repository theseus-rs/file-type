use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851836: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_836,
        source_type: SourceType::Wikidata,
        name: "B-EM Snapshot",
        extensions: &["snp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x45, 0x4D, 0x53, 0x4E, 0x41, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
