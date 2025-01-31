use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119786488: FileFormat = FileFormat {
    id: 119_786_488,
    puid: "wikidata/119786488",
    name: "MasterCook Export File",
    extensions: &["mx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
