use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49799747: FileFormat = FileFormat {
    id: 49_799_747,
    puid: "wikidata/49799747",
    name: "Adobe Portable Document Catalog Index File, version 3.2",
    extensions: &["pdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
