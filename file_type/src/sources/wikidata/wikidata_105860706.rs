use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860706: FileFormat = FileFormat {
    id: 105_860_706,
    puid: "wikidata/105860706",
    name: "Ren'Py Archive (v2)",
    extensions: &["rpa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x41, 0x2D, 0x32, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
