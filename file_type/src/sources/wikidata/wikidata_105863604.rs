use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863604: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_604,
        source_type: SourceType::Wikidata,
        name: "Wolfpack Mission",
        extensions: &["mis"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x50, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
