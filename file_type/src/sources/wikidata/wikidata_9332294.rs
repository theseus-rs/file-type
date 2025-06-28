use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_9332294: FileType = FileType {
    file_format: &FileFormat {
        id: 9_332_294,
        source_type: SourceType::Wikidata,
        name: "SubRip text file format",
        extensions: &["srt"],
        media_types: &["application/x-subrip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
