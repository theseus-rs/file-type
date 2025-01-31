use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117850207: FileFormat = FileFormat {
    id: 117_850_207,
    puid: "wikidata/117850207",
    name: "Xerox MicroFax",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
