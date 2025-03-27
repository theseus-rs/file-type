use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27866048: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_048,
        source_type: SourceType::Wikidata,
        name: "Canon Original RAW, version 2",
        extensions: &["cr2"],
        media_types: &["image/x-canon-cr2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x2A, 0x00, 0x10, 0x00, 0x00, 0x00, 0x43, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
