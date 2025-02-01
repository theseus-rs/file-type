use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62414916: FileFormat = FileFormat {
    id: 62_414_916,
    puid: "wikidata/62414916",
    name: "Outlook Express Folder Database",
    extensions: &["dbx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
