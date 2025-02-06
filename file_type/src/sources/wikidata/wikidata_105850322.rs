use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850322: FileFormat = FileFormat {
    id: 105_850_322,
    source_type: SourceType::Wikidata,
    name: "Atlantis Word Processor encrypted document",
    extensions: &["cod"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x39, 0x38, 0x37, 0x44, 0x20, 0x54, 0x74, 0x63, 0x61, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
