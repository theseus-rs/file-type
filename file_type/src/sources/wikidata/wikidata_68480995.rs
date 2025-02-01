use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_68480995: FileFormat = FileFormat {
    id: 68_480_995,
    puid: "wikidata/68480995",
    name: "Kingsoft PowerWord Dictionary",
    extensions: &["dic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
