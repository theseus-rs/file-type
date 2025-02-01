use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128771060: FileFormat = FileFormat {
    id: 128_771_060,
    puid: "wikidata/128771060",
    name: "Chapel source code file",
    extensions: &["chpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
