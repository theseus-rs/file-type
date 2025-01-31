use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48105549: FileFormat = FileFormat {
    id: 48_105_549,
    puid: "wikidata/48105549",
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
