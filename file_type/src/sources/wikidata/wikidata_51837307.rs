use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51837307: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_307,
        source_type: SourceType::Wikidata,
        name: "IBM DisplayWrite DCA Text File",
        extensions: &["dca", "rft"],
        media_types: &["application/dca-rft", "application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x05, 0xE1, 0x03, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
