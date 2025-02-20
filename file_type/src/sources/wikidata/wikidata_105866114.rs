use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866114: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_114,
        source_type: SourceType::Wikidata,
        name: "PCE block device image",
        extensions: &["pimg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x4D, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
