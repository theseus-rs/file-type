use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851682: FileFormat = FileFormat {
    id: 105_851_682,
    puid: "wikidata/105851682",
    name: "WinAPE recorded session (simple)",
    extensions: &["snp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x56, 0x20, 0x2D, 0x20, 0x53, 0x4E, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
