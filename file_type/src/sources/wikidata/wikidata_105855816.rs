use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855816: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_816,
        source_type: SourceType::Wikidata,
        name: "Oric MFM disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x46, 0x4D, 0x5F, 0x44, 0x49, 0x53, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
