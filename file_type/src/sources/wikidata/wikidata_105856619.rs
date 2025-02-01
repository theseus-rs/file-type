use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856619: FileFormat = FileFormat {
    id: 105_856_619,
    puid: "wikidata/105856619",
    name: "Windows Metafile (old Win 3.x format)",
    extensions: &["wmf"],
    media_types: &["image/x-wmf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x09, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
