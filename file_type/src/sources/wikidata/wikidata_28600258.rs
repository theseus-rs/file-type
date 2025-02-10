use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28600258: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_258,
        source_type: SourceType::Wikidata,
        name: "ATR",
        extensions: &["atr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x96, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
