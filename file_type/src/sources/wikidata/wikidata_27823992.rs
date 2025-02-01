use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823992: FileFormat = FileFormat {
    id: 27_823_992,
    puid: "wikidata/27823992",
    name: "Maptech BSB documentation file, version 3.0",
    extensions: &["bsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
