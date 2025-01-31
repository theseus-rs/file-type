use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113841023: FileFormat = FileFormat {
    id: 113_841_023,
    puid: "wikidata/113841023",
    name: "JIFF",
    extensions: &["jif", "jiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
