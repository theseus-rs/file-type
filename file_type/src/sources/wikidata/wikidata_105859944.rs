use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859944: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_944,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts MAD video (inter-frame)",
        extensions: &["mad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x44, 0x6D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
