use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47519856: FileFormat = FileFormat {
    id: 47_519_856,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 8",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
