use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60413976: FileFormat = FileFormat {
    id: 60_413_976,
    source_type: SourceType::Wikidata,
    name: "Encase Image File Format/Expert Witness Compression Format",
    extensions: &["e01"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x56, 0x46, 0x09, 0x0D, 0x0A, 0xFF, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
