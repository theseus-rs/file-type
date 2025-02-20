use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866365: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_365,
        source_type: SourceType::Wikidata,
        name: "Paint.NET Image (v3)",
        extensions: &["pdn"],
        media_types: &["image/x-paintnet"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x44, 0x4E, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
