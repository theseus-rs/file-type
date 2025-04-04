use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_308,
        source_type: SourceType::Wikidata,
        name: "YGOPRO replay",
        extensions: &["yrp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x79, 0x72, 0x70, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
