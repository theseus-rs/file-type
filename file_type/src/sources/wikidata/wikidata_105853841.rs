use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853841: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_841,
        source_type: SourceType::Wikidata,
        name: "PPMZ compressed data",
        extensions: &["ppmz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x4D, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
