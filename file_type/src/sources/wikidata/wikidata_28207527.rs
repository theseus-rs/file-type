use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207527: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_527,
        source_type: SourceType::Wikidata,
        name: "Wavelet Scalar Quantization",
        extensions: &["wsq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xA0, 0xFF, 0xA8, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
