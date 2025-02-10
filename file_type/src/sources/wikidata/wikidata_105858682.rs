use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_682,
        source_type: SourceType::Wikidata,
        name: "Winlog Raster File bitmap",
        extensions: &["wrf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x52, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
