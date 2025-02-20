use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859936: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_936,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts ASF video (generic)",
        extensions: &["asf", "str"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x48, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
