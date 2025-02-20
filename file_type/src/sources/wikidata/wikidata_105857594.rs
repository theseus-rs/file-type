use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857594: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_594,
        source_type: SourceType::Wikidata,
        name: "B-DOS bootable disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0x42, 0x2D, 0x44, 0x4F, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
