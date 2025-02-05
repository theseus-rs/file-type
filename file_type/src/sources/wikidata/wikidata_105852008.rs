use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852008: FileFormat = FileFormat {
    id: 105_852_008,
    source_type: SourceType::Wikidata,
    name: "FDM downloads data",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x44, 0x4D, 0x20, 0x44, 0x6F, 0x77, 0x6E, 0x6C, 0x6F, 0x61, 0x64, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
