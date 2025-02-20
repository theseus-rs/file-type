use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861072: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_072,
        source_type: SourceType::Wikidata,
        name: "Linden Lab Structured Data",
        extensions: &["llsd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x6C, 0x6C, 0x73, 0x64, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
