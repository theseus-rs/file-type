use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857138: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_138,
        source_type: SourceType::Wikidata,
        name: "HDF IDE hard disk image",
        extensions: &["hdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x2D, 0x49, 0x44, 0x45, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
