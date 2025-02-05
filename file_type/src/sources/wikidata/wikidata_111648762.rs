use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111648762: FileFormat = FileFormat {
    id: 111_648_762,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Scrapbook Page File",
    extensions: &["sbp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
