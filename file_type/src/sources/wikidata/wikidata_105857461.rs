use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857461: FileFormat = FileFormat {
    id: 105_857_461,
    puid: "wikidata/105857461",
    name: "VersaCAD 2D Library (MS-DOS)",
    extensions: &["2dl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x08, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
