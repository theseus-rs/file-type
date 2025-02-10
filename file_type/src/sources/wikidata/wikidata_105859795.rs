use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859795: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_795,
        source_type: SourceType::Wikidata,
        name: "Starsiege Tribes game data archive",
        extensions: &["vol"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x56, 0x4F, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
