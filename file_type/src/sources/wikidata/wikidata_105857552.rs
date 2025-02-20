use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857552: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_552,
        source_type: SourceType::Wikidata,
        name: "EZ-DiskCopy PRO disk image",
        extensions: &["rim"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x5A, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
