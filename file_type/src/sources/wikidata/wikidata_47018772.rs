use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47018772: FileFormat = FileFormat {
    id: 47_018_772,
    puid: "wikidata/47018772",
    name: "PageMaker Document file format, version 6.5",
    extensions: &["p65"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
