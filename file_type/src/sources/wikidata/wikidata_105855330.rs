use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855330: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_330,
        source_type: SourceType::Wikidata,
        name: "Full Disk Image",
        extensions: &["fdi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x44, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
