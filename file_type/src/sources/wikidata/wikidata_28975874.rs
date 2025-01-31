use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975874: FileFormat = FileFormat {
    id: 28_975_874,
    puid: "wikidata/28975874",
    name: "OOGL TLIST Group file",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
