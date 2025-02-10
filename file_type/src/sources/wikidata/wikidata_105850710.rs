use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_710,
        source_type: SourceType::Wikidata,
        name: "Chinese KuGou ResourCe (KuGou Music lyric)",
        extensions: &["krc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x72, 0x63, 0x31, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
