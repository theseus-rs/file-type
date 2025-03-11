use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859950: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_950,
        source_type: SourceType::Wikidata,
        name: "Chromium Visited Links history",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4C, 0x6E, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
