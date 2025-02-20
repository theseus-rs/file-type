use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859366: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_366,
        source_type: SourceType::Wikidata,
        name: "Quartus Compiler Setting File",
        extensions: &["csf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x45, 0x52, 0x5F, 0x53, 0x45, 0x54,
                        0x54, 0x49, 0x4E, 0x47, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
