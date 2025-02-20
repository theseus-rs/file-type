use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856842: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_842,
        source_type: SourceType::Wikidata,
        name: "G64 1541 raw disk image",
        extensions: &["g64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x43, 0x52, 0x2D, 0x31, 0x35, 0x34, 0x31, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
