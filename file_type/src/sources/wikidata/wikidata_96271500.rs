use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96271500: FileFormat = FileFormat {
    id: 96_271_500,
    puid: "wikidata/96271500",
    name: "FlagMaker file format",
    extensions: &["flag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
