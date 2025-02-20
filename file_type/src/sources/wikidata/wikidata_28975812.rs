use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975812: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_812,
        source_type: SourceType::Wikidata,
        name: "International Laser Display Association image data transfer format",
        extensions: &["ild"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x4C, 0x44, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
