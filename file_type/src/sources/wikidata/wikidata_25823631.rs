use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25823631: FileFormat = FileFormat {
    id: 25_823_631,
    source_type: SourceType::Wikidata,
    name: "MapCSS",
    extensions: &["mapcss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
