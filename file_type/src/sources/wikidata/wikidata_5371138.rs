use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5371138: FileType = FileType {
    file_format: &FileFormat {
        id: 5_371_138,
        source_type: SourceType::Wikidata,
        name: "MPQ",
        extensions: &["mpq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x51, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
