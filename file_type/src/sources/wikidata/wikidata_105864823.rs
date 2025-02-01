use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864823: FileFormat = FileFormat {
    id: 105_864_823,
    puid: "wikidata/105864823",
    name: "PC-File data (gen)",
    extensions: &["hdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x46, 0x3A, 0x64, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
