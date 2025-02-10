use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857458: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_458,
        source_type: SourceType::Wikidata,
        name: "CP Backup saved data (v7.x)",
        extensions: &["001"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x53, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
