use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859198: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_198,
        source_type: SourceType::Wikidata,
        name: "CUPS Raster bitmap (v1, LE)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x53, 0x61, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
