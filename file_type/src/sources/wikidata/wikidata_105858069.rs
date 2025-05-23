use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858069: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_069,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto IV Item Definition",
        extensions: &["ide"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6F, 0x62, 0x6A, 0x73, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
