use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858262: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_262,
        source_type: SourceType::Wikidata,
        name: "R.A.G.E. Driver",
        extensions: &["rge"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD7, 0xAD, 0x64, 0xAB, 0xFB, 0x69])],
                },
            }],
        }],
        related_formats: &[],
    },
};
