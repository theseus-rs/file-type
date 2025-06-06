use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859620: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_620,
        source_type: SourceType::Wikidata,
        name: "VersaForm form",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x42, 0x4C, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
