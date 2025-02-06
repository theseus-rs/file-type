use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62414916: FileFormat = FileFormat {
    id: 62_414_916,
    source_type: SourceType::Wikidata,
    name: "Outlook Express Folder Database",
    extensions: &["dbx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCF, 0xAD, 0x12, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
