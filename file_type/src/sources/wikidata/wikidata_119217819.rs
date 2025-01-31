use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119217819: FileFormat = FileFormat {
    id: 119_217_819,
    puid: "wikidata/119217819",
    name: "QuickBooks Portable Company File",
    extensions: &["qbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
