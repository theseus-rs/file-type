use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73750947: FileType = FileType {
    file_format: &FileFormat {
        id: 73_750_947,
        source_type: SourceType::Wikidata,
        name: "Q-emulator Configuration",
        extensions: &["qcf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x61, 0x6D, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
