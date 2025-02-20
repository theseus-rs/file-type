use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_612,
        source_type: SourceType::Wikidata,
        name: "Statistica MFM data",
        extensions: &["mfm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x53, 0x34, 0x4C, 0xD9, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
