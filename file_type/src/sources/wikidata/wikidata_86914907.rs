use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86914907: FileType = FileType {
    file_format: &FileFormat {
        id: 86_914_907,
        source_type: SourceType::Wikidata,
        name: "IESNA LM-63 Photometric Data File",
        extensions: &["ies"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x45, 0x53, 0x4E, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
