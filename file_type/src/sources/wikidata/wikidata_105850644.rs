use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850644: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_644,
        source_type: SourceType::Wikidata,
        name: "ChemDraw Template",
        extensions: &["ctr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x4D, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
