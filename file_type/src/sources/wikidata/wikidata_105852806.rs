use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852806: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_806,
        source_type: SourceType::Wikidata,
        name: "Superbase Query definition",
        extensions: &["sbq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
