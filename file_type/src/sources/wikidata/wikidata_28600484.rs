use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600484: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_484,
        source_type: SourceType::Wikidata,
        name: "DVDisaster Error Correction File",
        extensions: &["ecc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x64, 0x76, 0x64, 0x69, 0x73, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
