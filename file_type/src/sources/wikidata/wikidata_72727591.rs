use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72727591: FileFormat = FileFormat {
    id: 72_727_591,
    puid: "wikidata/72727591",
    name: "Juno address book",
    extensions: &["nv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
