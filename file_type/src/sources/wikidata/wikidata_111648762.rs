use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111648762: FileFormat = FileFormat {
    id: 111_648_762,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Scrapbook Page File",
    extensions: &["sbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
