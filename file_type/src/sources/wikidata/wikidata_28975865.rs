use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975865: FileFormat = FileFormat {
    id: 28_975_865,
    puid: "wikidata/28975865",
    name: "OOGL VECT file",
    extensions: &["vect"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
