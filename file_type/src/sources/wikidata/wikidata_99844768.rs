use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_99844768: FileFormat = FileFormat {
    id: 99_844_768,
    source_type: SourceType::Wikidata,
    name: "MicroStation Base File",
    extensions: &["bse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
