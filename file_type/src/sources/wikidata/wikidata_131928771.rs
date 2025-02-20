use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131928771: FileType = FileType {
    file_format: &FileFormat {
        id: 131_928_771,
        source_type: SourceType::Wikidata,
        name: "ConceptDraw Document",
        extensions: &["cdd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4E, 0x43, 0x45, 0x50, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
