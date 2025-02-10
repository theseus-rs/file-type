use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_224,
        source_type: SourceType::Wikidata,
        name: "Outerra Package",
        extensions: &["pkg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4F, 0x50, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
