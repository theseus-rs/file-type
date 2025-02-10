use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851111: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_111,
        source_type: SourceType::Wikidata,
        name: "Tree Generator 3D tree",
        extensions: &["tgf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x47, 0x46, 0x20, 0x62, 0x79, 0x20, 0x54, 0x72, 0x65, 0x65, 0x20,
                        0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x6F, 0x72, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
