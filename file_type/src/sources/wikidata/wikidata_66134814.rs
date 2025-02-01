use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66134814: FileFormat = FileFormat {
    id: 66_134_814,
    puid: "wikidata/66134814",
    name: "Access Add-in file format",
    extensions: &["mda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
