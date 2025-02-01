use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129825037: FileFormat = FileFormat {
    id: 129_825_037,
    puid: "wikidata/129825037",
    name: "Io source code file",
    extensions: &["io"],
    media_types: &["text/x-iosrc"],
    internal_signatures: &[],
    related_formats: &[],
};
