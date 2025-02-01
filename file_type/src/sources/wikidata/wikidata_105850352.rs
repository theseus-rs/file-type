use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850352: FileFormat = FileFormat {
    id: 105_850_352,
    puid: "wikidata/105850352",
    name: "QuickLink Fax Cover",
    extensions: &["cvr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x53, 0x49, 0x43, 0x56, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
