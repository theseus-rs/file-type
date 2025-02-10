use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855281: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_281,
        source_type: SourceType::Wikidata,
        name: "Farandole F3R blocked linear module format",
        extensions: &["f3r"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x33, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
