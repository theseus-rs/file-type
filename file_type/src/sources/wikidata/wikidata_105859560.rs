use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859560: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_560,
        source_type: SourceType::Wikidata,
        name: "HHE1 video",
        extensions: &["hhe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x48, 0x45, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
