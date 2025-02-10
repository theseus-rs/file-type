use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856908: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_908,
        source_type: SourceType::Wikidata,
        name: "GFA-BASIC MS-DOS tokenized source",
        extensions: &["gfa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x46, 0x41, 0x2D, 0x42, 0x41, 0x53, 0x49, 0x43, 0x20, 0x4D, 0x53,
                        0x2D, 0x44, 0x4F, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
