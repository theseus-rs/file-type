use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205751: FileFormat = FileFormat {
    id: 28_205_751,
    puid: "wikidata/28205751",
    name: "BCIF",
    extensions: &["bcif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
