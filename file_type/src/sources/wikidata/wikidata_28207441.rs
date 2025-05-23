use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207441: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_441,
        source_type: SourceType::Wikidata,
        name: "Khoros Visualization image, version 2.0",
        extensions: &["vif", "viff", "xv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x13, 0x57, 0x9A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
