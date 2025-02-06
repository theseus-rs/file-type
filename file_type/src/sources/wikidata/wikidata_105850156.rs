use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850156: FileFormat = FileFormat {
    id: 105_850_156,
    source_type: SourceType::Wikidata,
    name: "Windows Clipboard (NT)",
    extensions: &["clp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0xC3])],
            },
        }],
    }],
    related_formats: &[],
};
