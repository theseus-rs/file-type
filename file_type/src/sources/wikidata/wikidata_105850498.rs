use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850498: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_498,
        source_type: SourceType::Wikidata,
        name: "NetCDF (network Common Data Form)",
        extensions: &["cdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x65, 0x74, 0x63, 0x64, 0x66, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
