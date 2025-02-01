use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34743262: FileFormat = FileFormat {
    id: 34_743_262,
    puid: "wikidata/34743262",
    name: "Softdisk Text Compressor",
    extensions: &["ctx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x43, 0x54, 0x30, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
