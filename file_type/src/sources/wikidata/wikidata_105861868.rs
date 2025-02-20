use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861868: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_868,
        source_type: SourceType::Wikidata,
        name: "Synder SNG-Player module",
        extensions: &["sng"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x59, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
