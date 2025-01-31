use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975892: FileFormat = FileFormat {
    id: 28_975_892,
    puid: "wikidata/28975892",
    name: "Force Control File",
    extensions: &["frc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
