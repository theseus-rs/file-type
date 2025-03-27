use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445592: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_592,
        source_type: SourceType::Wikidata,
        name: "AMOS Memory Bank",
        extensions: &["abk"],
        media_types: &["application/x-amos-memorybank"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6D, 0x42, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
