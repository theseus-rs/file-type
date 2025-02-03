use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50498413: FileFormat = FileFormat {
    id: 50_498_413,
    source_type: SourceType::Wikidata,
    name: "Draco File Format",
    extensions: &["drc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
