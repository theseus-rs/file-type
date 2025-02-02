use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73504589: FileFormat = FileFormat {
    id: 73_504_589,
    source_type: SourceType::Wikidata,
    name: "CorelFlow",
    extensions: &["cfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
