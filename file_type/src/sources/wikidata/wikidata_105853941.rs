use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853941: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_941,
        source_type: SourceType::Wikidata,
        name: "UltraCompressor 2 archive split Part",
        extensions: &["p01", "p02", "p03"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x14, 0x02, 0x0F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
