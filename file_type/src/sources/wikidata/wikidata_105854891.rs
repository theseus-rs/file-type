use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854891: FileFormat = FileFormat {
    id: 105_854_891,
    puid: "wikidata/105854891",
    name: "HPACK compressed archive",
    extensions: &["hpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
