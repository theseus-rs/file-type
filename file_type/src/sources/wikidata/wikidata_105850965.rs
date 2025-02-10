use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850965: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_965,
        source_type: SourceType::Wikidata,
        name: "Binary Unicode conversion Table",
        extensions: &["tbl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x6E, 0x69, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
