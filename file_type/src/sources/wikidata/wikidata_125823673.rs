use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125823673: FileFormat = FileFormat {
    id: 125_823_673,
    puid: "wikidata/125823673",
    name: "Gzipped Tar File",
    extensions: &["tgz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
