use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551372: FileFormat = FileFormat {
    id: 28_551_372,
    puid: "wikidata/28551372",
    name: "Adobe Monitor Setup File",
    extensions: &["ams"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
