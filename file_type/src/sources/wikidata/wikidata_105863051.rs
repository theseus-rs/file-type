use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863051: FileFormat = FileFormat {
    id: 105_863_051,
    puid: "wikidata/105863051",
    name: "Microsoft Jet DB",
    extensions: &["mdb"],
    media_types: &["application/x-msaccess"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x20,
                    0x4A, 0x65, 0x74, 0x20, 0x44, 0x42, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
