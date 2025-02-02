use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121837535: FileFormat = FileFormat {
    id: 121_837_535,
    source_type: SourceType::Wikidata,
    name: "OPML File 1.x",
    extensions: &["opml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
