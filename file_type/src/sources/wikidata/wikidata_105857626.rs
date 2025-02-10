use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857626: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_626,
        source_type: SourceType::Wikidata,
        name: "LibDisk disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x53, 0x4B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
