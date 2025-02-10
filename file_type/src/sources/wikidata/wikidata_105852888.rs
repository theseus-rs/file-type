use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852888: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_888,
        source_type: SourceType::Wikidata,
        name: "ADAMEm Snapshot",
        extensions: &["snp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x44, 0x41, 0x4D, 0x45, 0x6D, 0x20, 0x73, 0x6E, 0x61, 0x70, 0x73,
                        0x68, 0x6F, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
