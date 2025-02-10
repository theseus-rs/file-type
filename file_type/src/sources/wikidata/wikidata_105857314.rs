use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857314: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_314,
        source_type: SourceType::Wikidata,
        name: "Central Point Software Help data",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x78, 0x48, 0x21, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
