use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857357: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_357,
        source_type: SourceType::Wikidata,
        name: "Easy CD Creator's label (v5)",
        extensions: &["jwl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x00, 0x4F, 0x00, 0x58, 0x00, 0x49, 0x00, 0x20, 0x00, 0x35, 0x00,
                        0x2E, 0x00, 0x30, 0x00, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
