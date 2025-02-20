use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95985299: FileType = FileType {
    file_format: &FileFormat {
        id: 95_985_299,
        source_type: SourceType::Wikidata,
        name: "Affymetrix CDF file format",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x43, 0x44, 0x46, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
