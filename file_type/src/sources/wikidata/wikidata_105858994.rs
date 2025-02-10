use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858994: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_994,
        source_type: SourceType::Wikidata,
        name: "App Inventor Blocks XML",
        extensions: &["bky"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x78, 0x6D, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
