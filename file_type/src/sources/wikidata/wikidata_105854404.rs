use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854404: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_404,
        source_type: SourceType::Wikidata,
        name: "SQWEZ compressed archive",
        extensions: &["sqz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x51, 0x57, 0x45, 0x5A, 0x20, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
