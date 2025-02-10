use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850304: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_304,
        source_type: SourceType::Wikidata,
        name: "Protext Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x52, 0x4F, 0x54, 0x45, 0x58, 0x54, 0x20, 0x43, 0x4F, 0x4E, 0x46,
                        0x49, 0x47, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x56, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
