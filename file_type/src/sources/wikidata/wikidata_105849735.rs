use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_735,
        source_type: SourceType::Wikidata,
        name: "Celestia Sphere displacement Mesh (with rem)",
        extensions: &["cms"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
