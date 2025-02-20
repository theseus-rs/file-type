use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851465: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_465,
        source_type: SourceType::Wikidata,
        name: "Garmin Training Center Database XML (V2)",
        extensions: &["tcx"],
        media_types: &["application/vnd.garmin.tcx+xml"],
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
