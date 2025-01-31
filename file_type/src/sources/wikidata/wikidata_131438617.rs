use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131438617: FileFormat = FileFormat {
    id: 131_438_617,
    puid: "wikidata/131438617",
    name: "Xtend file format",
    extensions: &["xtend"],
    media_types: &["text/x-xtend"],
    internal_signatures: &[],
    related_formats: &[],
};
