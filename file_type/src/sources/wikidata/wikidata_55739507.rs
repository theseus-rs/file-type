use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55739507: FileFormat = FileFormat {
    id: 55_739_507,
    puid: "wikidata/55739507",
    name: "Genbox Family History file format",
    extensions: &["GDB"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
