use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49798739: FileFormat = FileFormat {
    id: 49_798_739,
    puid: "wikidata/49798739",
    name: "Adobe Portable Document Catalog Index File, version 3",
    extensions: &["pdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
