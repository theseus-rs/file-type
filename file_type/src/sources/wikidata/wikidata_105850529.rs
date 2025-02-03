use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850529: FileFormat = FileFormat {
    id: 105_850_529,
    source_type: SourceType::Wikidata,
    name: "MAME Compressed Hard Disk image",
    extensions: &["chd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x48, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
