use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_16976440: FileType = FileType {
    file_format: &FileFormat {
        id: 16_976_440,
        source_type: SourceType::Wikidata,
        name: "Segmented Hyper Graphics",
        extensions: &["shg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6C, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
