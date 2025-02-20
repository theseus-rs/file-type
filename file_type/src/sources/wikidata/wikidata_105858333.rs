use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858333: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_333,
        source_type: SourceType::Wikidata,
        name: "Wise Installer executable",
        extensions: &["exe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
