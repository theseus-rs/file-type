use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855950: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_950,
        source_type: SourceType::Wikidata,
        name: "Project Dogwaffle animation (generic)",
        extensions: &["dwa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x57, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
