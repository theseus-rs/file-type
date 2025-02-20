use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866169: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_169,
        source_type: SourceType::Wikidata,
        name: "PlayStation Patch File (generic)",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
