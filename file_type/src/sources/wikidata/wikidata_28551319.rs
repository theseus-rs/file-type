use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551319: FileFormat = FileFormat {
    id: 28_551_319,
    puid: "wikidata/28551319",
    name: "Adobe Custom Kernel File",
    extensions: &["acf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
