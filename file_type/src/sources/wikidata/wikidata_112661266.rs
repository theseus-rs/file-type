use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661266: FileFormat = FileFormat {
    id: 112_661_266,
    puid: "wikidata/112661266",
    name: "Lightscape Preparation File",
    extensions: &["lp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
