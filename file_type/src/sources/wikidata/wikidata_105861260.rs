use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861260: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_260,
        source_type: SourceType::Wikidata,
        name: "Lingoes Dictionary",
        extensions: &["ld2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3F, 0x4C, 0x44, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
