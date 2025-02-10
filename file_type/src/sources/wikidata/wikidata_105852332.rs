use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852332: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_332,
        source_type: SourceType::Wikidata,
        name: "BML3MK5 snapshot",
        extensions: &["l3r"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x53, 0x55, 0x4D, 0x45, 0x5F, 0x42, 0x4D, 0x4C, 0x33, 0x4D,
                        0x4B, 0x35,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
