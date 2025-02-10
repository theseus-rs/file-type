use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_179,
        source_type: SourceType::Wikidata,
        name: "Highway Pursuit game data archive",
        extensions: &["hfd", "hgd", "hmd", "hod", "hsd", "hvd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x50, 0x44, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
