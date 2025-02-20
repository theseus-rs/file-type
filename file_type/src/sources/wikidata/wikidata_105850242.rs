use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_242,
        source_type: SourceType::Wikidata,
        name: "SMS Coastline data",
        extensions: &["cst"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x41, 0x53, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
