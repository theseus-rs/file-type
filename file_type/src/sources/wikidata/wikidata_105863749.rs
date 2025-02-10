use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863749: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_749,
        source_type: SourceType::Wikidata,
        name: "MVSTracker Instrument",
        extensions: &["ins"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x56, 0x53, 0x49, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
