use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853734: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_734,
        source_type: SourceType::Wikidata,
        name: "Quest Adventure Script (generic)",
        extensions: &["asl"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x27, 0x20, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
