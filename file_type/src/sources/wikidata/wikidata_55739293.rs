use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55739293: FileFormat = FileFormat {
    id: 55_739_293,
    puid: "wikidata/55739293",
    name: "CRAM file format, version 1",
    extensions: &["cram"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
