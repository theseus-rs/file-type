use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_352,
        source_type: SourceType::Wikidata,
        name: "QuickLink Fax Cover",
        extensions: &["cvr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x53, 0x49, 0x43, 0x56, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
