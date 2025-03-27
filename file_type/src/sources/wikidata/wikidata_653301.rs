use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_653301: FileType = FileType {
    file_format: &FileFormat {
        id: 653_301,
        source_type: SourceType::Wikidata,
        name: "MusicXML",
        extensions: &["musicxml", "mxl", "xml"],
        media_types: &["application/vnd.recordare.musicxml+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
