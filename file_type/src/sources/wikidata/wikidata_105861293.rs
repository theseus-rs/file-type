use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861293: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_293,
        source_type: SourceType::Wikidata,
        name: "Jeppesen/Mentor FliteLog Log",
        extensions: &["lbk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC8, 0x00, 0x79, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
