use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551337: FileFormat = FileFormat {
    id: 28_551_337,
    puid: "wikidata/28551337",
    name: "Adobe Duotone Options File",
    extensions: &["ado"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
