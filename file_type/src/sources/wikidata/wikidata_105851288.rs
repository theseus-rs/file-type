use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851288: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_288,
        source_type: SourceType::Wikidata,
        name: "Tempus Word Document (old)",
        extensions: &["twd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09, 0x54, 0x5F, 0x57, 0x4F, 0x52, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
