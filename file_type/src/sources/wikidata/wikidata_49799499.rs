use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49799499: FileFormat = FileFormat {
    id: 49_799_499,
    puid: "wikidata/49799499",
    name: "Adobe Portable Document Catalog Index File, version 3.1",
    extensions: &["pdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
