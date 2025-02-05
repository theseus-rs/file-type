use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_16683501: FileFormat = FileFormat {
    id: 16_683_501,
    source_type: SourceType::Wikidata,
    name: "OpenXML Spreadsheet Macro-Enabled",
    extensions: &["xlsm"],
    media_types: &["application/vnd.ms-excel.sheet.macroEnabled.12"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
