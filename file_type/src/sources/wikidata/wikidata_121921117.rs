use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121921117: FileFormat = FileFormat {
    id: 121_921_117,
    puid: "wikidata/121921117",
    name: "Ptex File Format",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
