use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_191,
        source_type: SourceType::Wikidata,
        name: "StarOffice Gallery theme",
        extensions: &["thm"],
        media_types: &["application/x-stargallery-thm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
