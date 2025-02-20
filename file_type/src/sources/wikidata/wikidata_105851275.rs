use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851275: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_275,
        source_type: SourceType::Wikidata,
        name: "HiJaak PCL soft font",
        extensions: &["tpf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x46, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
