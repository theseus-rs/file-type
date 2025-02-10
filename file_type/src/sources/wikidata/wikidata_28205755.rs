use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205755: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_755,
        source_type: SourceType::Wikidata,
        name: "Big Flexible Line Interpretation",
        extensions: &["bfli"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0x3B, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
