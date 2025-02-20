use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855172: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_172,
        source_type: SourceType::Wikidata,
        name: "File-Type Image",
        extensions: &["fti"],
        media_types: &[],
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
