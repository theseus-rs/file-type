use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861802: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_802,
        source_type: SourceType::Wikidata,
        name: "EdLib module",
        extensions: &["d00", "d01", "s01"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x43, 0x48, 0x26, 0x02, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
