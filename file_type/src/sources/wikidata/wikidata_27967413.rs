use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967413: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_413,
        source_type: SourceType::Wikidata,
        name: "DOSBox Raw OPL",
        extensions: &["dro"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x42, 0x52, 0x41, 0x57, 0x4F, 0x50, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
