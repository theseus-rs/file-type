use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17029350: FileFormat = FileFormat {
    id: 17_029_350,
    source_type: SourceType::Wikidata,
    name: "Image Cytometry Standard",
    extensions: &["ics", "ids"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
