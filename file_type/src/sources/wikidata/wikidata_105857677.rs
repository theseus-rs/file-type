use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857677: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_677,
        source_type: SourceType::Wikidata,
        name: "SamDOS v2 bootable disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x13, 0x73, 0x61, 0x6D, 0x64, 0x6F, 0x73, 0x32, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
