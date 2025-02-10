use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_131,
        source_type: SourceType::Wikidata,
        name: "Flow Cytometry Standard format",
        extensions: &["fcs"],
        media_types: &["application/vnd.isac.fcs"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x43, 0x53, 0x32, 0x2E, 0x30, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
