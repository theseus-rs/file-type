use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858925: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_925,
        source_type: SourceType::Wikidata,
        name: "Xerox EDMICS-MMR bitmap",
        extensions: &["ed", "mmr"],
        media_types: &["image/vnd.fujixerox.edmics-mmr"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
