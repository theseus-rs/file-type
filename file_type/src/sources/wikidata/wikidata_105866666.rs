use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866666: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_666,
        source_type: SourceType::Wikidata,
        name: "Palm DiddleBug sketch",
        extensions: &["pdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x44, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
