use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849864: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_864,
        source_type: SourceType::Wikidata,
        name: "Source Insight Custom Language File",
        extensions: &["clf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x09, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
