use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850005: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_005,
        source_type: SourceType::Wikidata,
        name: "Xilinx Core Generator System Project",
        extensions: &["cgp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x45, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
