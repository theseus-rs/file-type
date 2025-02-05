use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863261: FileFormat = FileFormat {
    id: 105_863_261,
    source_type: SourceType::Wikidata,
    name: "MapWindow Symbols",
    extensions: &["mwsymb"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x61, 0x70, 0x57, 0x69, 0x6E, 0x47, 0x49, 0x53, 0x20, 0x4F, 0x63,
                    0x78, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
