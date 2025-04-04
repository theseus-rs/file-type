use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51993886: FileType = FileType {
    file_format: &FileFormat {
        id: 51_993_886,
        source_type: SourceType::Wikidata,
        name: "IBM DisplayWrite Final Form Text File",
        extensions: &["fft"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2B, 0xD2, 0x04, 0x85, 0x00, 0x00, 0x2B, 0xD1, 0x07, 0x05, 0x00, 0x1A,
                        0x00, 0x90, 0x01, 0x2B, 0xD1,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
