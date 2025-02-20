use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856023: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_023,
        source_type: SourceType::Wikidata,
        name: "Dune Firmware File",
        extensions: &["dff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x55, 0x4E, 0x45, 0x20, 0x46, 0x49, 0x52, 0x4D, 0x57, 0x41, 0x52,
                        0x45, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
