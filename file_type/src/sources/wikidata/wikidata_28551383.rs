use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551383: FileFormat = FileFormat {
    id: 28_551_383,
    puid: "wikidata/28551383",
    name: "Adobe Replace Color/Color Range File",
    extensions: &["axt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
