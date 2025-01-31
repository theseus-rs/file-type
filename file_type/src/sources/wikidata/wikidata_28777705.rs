use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777705: FileFormat = FileFormat {
    id: 28_777_705,
    puid: "wikidata/28777705",
    name: "MyHeritage Family Tree Builder",
    extensions: &["zed"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
