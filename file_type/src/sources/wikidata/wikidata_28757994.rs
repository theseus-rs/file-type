use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757994: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_994,
        source_type: SourceType::Wikidata,
        name: "Imagine Texture File",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x51])],
                },
            }],
        }],
        related_formats: &[],
    },
};
