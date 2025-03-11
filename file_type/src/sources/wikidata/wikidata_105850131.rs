use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_131,
        source_type: SourceType::Wikidata,
        name: "Clio measurement (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0B, 0x41, 0x55, 0x44, 0x49, 0x4F, 0x4D, 0x41, 0x54, 0x49, 0x43, 0x41,
                        0x04, 0x43, 0x4C, 0x49, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
