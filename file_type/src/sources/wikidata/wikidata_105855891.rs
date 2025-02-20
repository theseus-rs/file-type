use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855891: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_891,
        source_type: SourceType::Wikidata,
        name: "ACT Apricot disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x43, 0x54, 0x20, 0x41, 0x70, 0x72, 0x69, 0x63, 0x6F, 0x74, 0x20,
                        0x64, 0x69, 0x73, 0x6B, 0x20, 0x69, 0x6D, 0x61, 0x67, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
