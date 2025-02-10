use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_936,
        source_type: SourceType::Wikidata,
        name: "Dark Forces Game data archive",
        extensions: &["gob"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4F, 0x42, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
