use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205674: FileFormat = FileFormat {
    id: 28_205_674,
    source_type: SourceType::Wikidata,
    name: "Alpha Microsystems BMP",
    extensions: &["bmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFF, 0x00, 0x01, 0x64, 0x00, 0x00, 0x00, 0x03, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
