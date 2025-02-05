use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47519856: FileFormat = FileFormat {
    id: 47_519_856,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 8",
    extensions: &["ppp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
