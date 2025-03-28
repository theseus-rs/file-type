use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_653,
        source_type: SourceType::Wikidata,
        name: "PlayStation Sequence data format",
        extensions: &["seq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x45, 0x51, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
