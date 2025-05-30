use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205564: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_564,
        source_type: SourceType::Wikidata,
        name: "Nokia Picture Message",
        extensions: &["npm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x50, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
