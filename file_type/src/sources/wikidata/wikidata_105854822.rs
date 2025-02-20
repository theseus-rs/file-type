use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854822: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_822,
        source_type: SourceType::Wikidata,
        name: "packMP3 compressed MP3 audio",
        extensions: &["pmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
