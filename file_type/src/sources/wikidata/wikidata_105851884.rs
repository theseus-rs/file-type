use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_884,
        source_type: SourceType::Wikidata,
        name: "Paradox sort definition",
        extensions: &["sor"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x72, 0x61, 0x64, 0x6F, 0x78, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
