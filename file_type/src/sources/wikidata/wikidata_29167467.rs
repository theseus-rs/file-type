use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167467: FileFormat = FileFormat {
    id: 29_167_467,
    puid: "wikidata/29167467",
    name: "OME-XML",
    extensions: &["ome.xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
