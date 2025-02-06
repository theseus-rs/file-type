use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55753055: FileFormat = FileFormat {
    id: 55_753_055,
    source_type: SourceType::Wikidata,
    name: "Redcode Metadata File",
    extensions: &["rmd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
