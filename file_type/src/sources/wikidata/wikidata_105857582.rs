use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857582: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_582,
        source_type: SourceType::Wikidata,
        name: "MasterDOS v1.x bootable disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0x4D, 0x44, 0x4F, 0x53, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
