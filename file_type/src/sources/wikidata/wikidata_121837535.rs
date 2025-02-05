use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121837535: FileFormat = FileFormat {
    id: 121_837_535,
    source_type: SourceType::Wikidata,
    name: "OPML File 1.x",
    extensions: &["opml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
