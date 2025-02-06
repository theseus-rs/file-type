use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861775: FileFormat = FileFormat {
    id: 105_861_775,
    source_type: SourceType::Wikidata,
    name: "Moxcel spreadsheet",
    extensions: &["mxl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x58, 0x43, 0x45, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
