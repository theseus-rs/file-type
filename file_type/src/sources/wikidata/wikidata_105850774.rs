use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850774: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_774,
        source_type: SourceType::Wikidata,
        name: "Klystrack Instrument",
        extensions: &["ki"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x79, 0x64, 0x21, 0x69, 0x6E, 0x73, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
