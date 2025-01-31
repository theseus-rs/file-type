use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009733: FileFormat = FileFormat {
    id: 111_009_733,
    puid: "wikidata/111009733",
    name: "PrintMaster Note Card File format",
    extensions: &["not"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
