use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17029350: FileFormat = FileFormat {
    id: 17_029_350,
    source_type: SourceType::Wikidata,
    name: "Image Cytometry Standard",
    extensions: &["ics", "ids"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
