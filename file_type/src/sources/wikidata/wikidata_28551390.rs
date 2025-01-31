use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551390: FileFormat = FileFormat {
    id: 28_551_390,
    puid: "wikidata/28551390",
    name: "Adobe Selective Color File",
    extensions: &["asv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
