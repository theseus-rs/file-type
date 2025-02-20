use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866167: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_167,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal Symbol Table",
        extensions: &["psm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
