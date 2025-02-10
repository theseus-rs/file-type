use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858568: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_568,
        source_type: SourceType::Wikidata,
        name: "AMI BIOS logo/splash bitmap",
        extensions: &["grfx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x52, 0x46, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
