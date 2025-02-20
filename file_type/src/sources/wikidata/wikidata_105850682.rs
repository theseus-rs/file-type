use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_682,
        source_type: SourceType::Wikidata,
        name: "Prisoner Of Ice game data archive",
        extensions: &["kro"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x75, 0x72, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
