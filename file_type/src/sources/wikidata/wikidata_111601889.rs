use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111601889: FileFormat = FileFormat {
    id: 111_601_889,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign Document, version CC 2019",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
