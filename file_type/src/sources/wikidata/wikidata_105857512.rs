use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857512: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_512,
        source_type: SourceType::Wikidata,
        name: "Dalton's Disk Disintegrator Pro disk image",
        extensions: &["ddd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0xC9, 0xBF, 0xD0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
