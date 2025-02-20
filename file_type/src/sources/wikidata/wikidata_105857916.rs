use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857916: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_916,
        source_type: SourceType::Wikidata,
        name: "VMware 4 Virtual Disk (monolitic)",
        extensions: &["vmdk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x44, 0x4D, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
