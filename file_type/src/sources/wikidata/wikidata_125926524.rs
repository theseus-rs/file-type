use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125926524: FileFormat = FileFormat {
    id: 125_926_524,
    puid: "wikidata/125926524",
    name: "Solidworks Drawing Sheet",
    extensions: &["slddrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
