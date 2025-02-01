use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_20191913: FileFormat = FileFormat {
    id: 20_191_913,
    puid: "wikidata/20191913",
    name: "Apple Help File Format",
    extensions: &["lproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
